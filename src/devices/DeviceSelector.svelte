<script lang="ts">
  import * as Select from '$lib/components/ui/select'
  import type { FftunerDevice } from './utils'
  import { fetchDevices, setDevice } from './utils'
  import { info, error, debug } from '@tauri-apps/plugin-log'
  import { untrack } from 'svelte'

  let availableDevices: FftunerDevice[] = $state([])
  let selectedIndex: number | null = $state(null)
  let selectedDevice: FftunerDevice | null = $derived.by(() => {
    const devs = untrack(() => availableDevices)
    if (selectedIndex !== null) {
      return devs[selectedIndex]
    }
    return null
  })

  let openState = $state(false)

  function onOpenChange() {
    if (!openState) {
      fetchDevices()
        .then((devices) => {
          availableDevices = devices
        })
        .catch((err) => {
          error('Error fetching devices: ' + err)
          return []
        })
    }
    openState = !openState
  }

  function setSelected(index: number) {
    setDevice(index)
    openState = false
  }
</script>

<Select.Root open={openState} {onOpenChange}>
  <Select.Trigger class="w-[180px] text-zinc-400">
    <Select.Value
      placeholder={selectedDevice?.name ?? 'Select a device'}
      class="text-zinc-400"
    />
  </Select.Trigger>
  <Select.Content class="bg-zinc-800 text-zinc-300">
    <Select.Group class="bg-zinc-800 text-zinc-300">
      <Select.Label class="bg-zinc-800 text-zinc-300">Devices</Select.Label>
      {#await fetchDevices()}
        <Select.Item
          value="loading"
          label="Loading..."
          class="bg-zinc-800 text-zinc-400"
        />
      {:then}
        {#each availableDevices as device, index}
          <Select.Item
            value={device.selectorId}
            label={device.name}
            class="bg-zinc-800 text-zinc-400"
            on:click={() => {
              setSelected(index)
            }}
          />
        {/each}
      {/await}
    </Select.Group>
  </Select.Content>
  <Select.Input name="device" />
</Select.Root>
