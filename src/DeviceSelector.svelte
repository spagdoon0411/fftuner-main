<script lang="ts">
  import * as Select from '$lib/components/ui/select'
  import { invoke } from '@tauri-apps/api/core'
  import { emit } from '@tauri-apps/api/event'
  import { untrack, onMount } from 'svelte'
  import { info, error } from '@tauri-apps/plugin-log'

  interface FftunerDevice {
    name: string
    hostName: string
    selectorId: string
  }

  let openState = $state(false)
  let availableDevices: FftunerDevice[] = $state([])
  let selected: string | null = $state(null)
  let selectedValue = $derived.by(() => {
    // Prevent refresh on availableDevices update
    let aDevs = untrack(() => availableDevices)
    return (
      aDevs?.find((device: FftunerDevice) => device.selectorId === selected)
        ?.hostName || 'Select a device'
    )
  })

  function triggerDeviceQuery() {
    availableDevices = []
    invoke('query_devices')
      .then((response) => {
        info(('Devices response' + response) as string)
        let resObj = JSON.parse(response as string)
        availableDevices = resObj.devices
        selected = resObj.setId
        // Triggers an update of selectedValue
      })
      .catch((err) => {
        error('Error querying for devices: ' + err)
      })
  }

  onMount(() => {
    info('Querying for devices on mount')
    triggerDeviceQuery()
  })

  function onOpenChange() {
    if (!openState) {
      triggerDeviceQuery()
    }
    openState = !openState
  }

  // TODO - create a response to set to the default device

  // A coerced selection event should also update availableDevices, passing back
  // a new availableDevices array along with the selected index

  // A coerced selection is fundamentally different from a user selection; information
  // flows in a different direction.

  // Frontend --- (update, index) ---> Backend
  // Backend --- (update, index) ---> Frontend

  // Setting a selected device
  function setSelected(selectorId: string) {
    selected = selectorId
    emit('select_device', { selector_id: selected })
  }
</script>

<Select.Root open={openState} {onOpenChange}>
  <Select.Trigger class="w-[180px] text-zinc-400" on:click={triggerDeviceQuery}>
    <Select.Value placeholder={selectedValue} class="text-zinc-400" />
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
  <Select.Input name="device" on:click={triggerDeviceQuery} />
</Select.Root>
