use model_derive::model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    AwsSettings, BootSettings, BootstrapContainer, ContainerRuntimeSettings, DnsSettings,
    HostContainer, KernelSettings, KubernetesSettings, MetricsSettings, NetworkSettings,
    NtpSettings, OciDefaults, OciHooks, PemCertificate, RegistrySettings, UpdatesSettings,
};
use modeled_types::Identifier;

// Note: we have to use 'rename' here because the top-level Settings structure is the only one
// that uses its name in serialization; internal structures use the field name that points to it
#[model(rename = "settings", impl_default = true)]
struct Settings {
    motd: settings_extension_motd::MotdV1,
    kubernetes: KubernetesSettings,
    updates: UpdatesSettings,
    host_containers: HashMap<Identifier, HostContainer>,
    bootstrap_containers: HashMap<Identifier, BootstrapContainer>,
    ntp: NtpSettings,
    network: NetworkSettings,
    kernel: KernelSettings,
    aws: AwsSettings,
    boot: BootSettings,
    metrics: MetricsSettings,
    pki: HashMap<Identifier, PemCertificate>,
    container_registry: RegistrySettings,
    oci_defaults: OciDefaults,
    oci_hooks: OciHooks,
    dns: DnsSettings,
    container_runtime: ContainerRuntimeSettings,
}
