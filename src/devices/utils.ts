import { info, error, debug } from '@tauri-apps/plugin-log'
import { invoke } from '@tauri-apps/api/core'

export type FftunerDevice = {
  name: string
  hostName: string
  selectorId: string
}

export async function fetchDevices(): Promise<FftunerDevice[]> {
  debug('Getting devices')
  let devices: FftunerDevice[] = []

  const res = await invoke('get_devices').catch((e) => {
    error('Error getting devices: ' + e)
    devices = []
    return devices
  })

  devices = JSON.parse(res as string) as FftunerDevice[]

  return devices
}
