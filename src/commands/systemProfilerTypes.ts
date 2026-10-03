// Rust の system_profiler.rs の ALLOWED_DATA_TYPES と一致させること。
// アプリ・フォント等の巨大・低速な型は選べない。

export const DEFAULT_SYSTEM_PROFILER_TYPE = "SPHardwareDataType";

export const SYSTEM_PROFILER_TYPES: string[] = [
  "SPHardwareDataType",
  "SPSoftwareDataType",
  "SPMemoryDataType",
  "SPStorageDataType",
  "SPDisplaysDataType",
  "SPPowerDataType",
  "SPNetworkDataType",
  "SPAirPortDataType",
  "SPBluetoothDataType",
  "SPUSBHostDataType",
  "SPThunderboltDataType",
  "SPAudioDataType",
  "SPCameraDataType",
  "SPPrintersDataType",
  "SPFirewallDataType",
  "SPNetworkLocationDataType",
  "SPInstallHistoryDataType",
  "SPStartupItemDataType",
  "SPConfigurationProfileDataType",
  "SPDeveloperToolsDataType",
  "SPDiagnosticsDataType",
  "SPNVMeDataType",
];
