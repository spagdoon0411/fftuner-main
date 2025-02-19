<script lang="ts">
  import * as Select from '$lib/components/ui/select'
  import type { FftunerDevice } from './utils'
  import { fetchDevices } from './utils'
  import { info, error, debug } from '@tauri-apps/plugin-log'

  let availableDevices: FftunerDevice[] = $state([])
  let selectedDevice: FftunerDevice | null = $state(null)

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

  function setSelected(selectorId: string) {
    selectedDevice =
      availableDevices?.find((device) => device.selectorId === selectorId) ??
      null
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
        {#each availableDevices as device}
          <Select.Item
            value={device.selectorId}
            label={device.name}
            class="bg-zinc-800 text-zinc-400"
            on:click={() => {
              setSelected(device.selectorId)
            }}
          />
        {/each}
      {/await}
    </Select.Group>
  </Select.Content>
  <Select.Input name="device" />
</Select.Root>
