<script lang="ts">
  import * as Select from '$lib/components/ui/select'
  import type { FftunerDevice } from './utils'
  import { getFftunerDevices } from './utils'

  let availableDevices: FftunerDevice[] = $state(getFftunerDevices())
  let selectedDevice: FftunerDevice | null = $state(null)

  function onDropDown() {
    availableDevices = getFftunerDevices()
  }

  let openState = $state(false)
  function onOpenChange() {
    if (!openState) {
      availableDevices = getFftunerDevices()
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
  <Select.Trigger class="w-[180px] text-zinc-400" on:click={onDropDown}>
    <Select.Value
      placeholder={selectedDevice?.name ?? 'Select a device'}
      class="text-zinc-400"
    />
  </Select.Trigger>
  <Select.Content class="bg-zinc-800 text-zinc-300">
    <Select.Group class="bg-zinc-800 text-zinc-300">
      <Select.Label class="bg-zinc-800 text-zinc-300">Devices</Select.Label>
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
    </Select.Group>
  </Select.Content>
  <Select.Input name="device" />
</Select.Root>
