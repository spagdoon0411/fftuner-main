import { info, error, debug } from '@tauri-apps/plugin-log'

export type FftunerDevice = {
  name: string
  hostName: string
  selectorId: string
}

const dummyDevices = [
  {
    name: 'Device 1',
    hostName: 'localhost',
    selectorId: 'device1',
  },
  {
    name: 'Device 2',
    hostName: 'localhost',
    selectorId: 'device2',
  },
  {
    name: 'Device 3',
    hostName: 'localhost',
    selectorId: 'device3',
  },
]

// Maps to cpal command to get devices
export function getFftunerDevices(): FftunerDevice[] {
  debug('Getting devices')
  return dummyDevices
}
