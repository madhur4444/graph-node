use prelude::*;

/// Type alias for data source IDs.
type DataSourceID = String;

/// Events emitted by a runtime host.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeHostEvent {
    /// An entity should be create or updated.
    EntitySet(DataSourceID, StoreKey, Entity),
    /// An entity should be removed.
    EntityRemoved(DataSourceID, StoreKey),
}

/// Common trait for runtime host implementations.
pub trait RuntimeHost: EventProducer<RuntimeHostEvent> {
    /// The data source definition the runtime is for.
    fn data_source_definition(&self) -> &DataSourceDefinition;
}

pub trait RuntimeHostBuilder {
    type Host: RuntimeHost;

    /// Build a new runtime host for a dataset.
    fn build(
        &mut self,
        data_source_definition: DataSourceDefinition,
        data_set: DataSet,
    ) -> Self::Host;
}