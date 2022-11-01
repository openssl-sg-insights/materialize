// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use mz_ore::metric;
use mz_ore::metrics::raw::{IntCounterVec, IntGaugeVec};
use mz_ore::metrics::{IntCounter, IntGauge, MetricsRegistry};

#[derive(Clone, Debug)]
pub struct MetricsConfig {
    active_connections: IntGaugeVec,
    connection_status: IntCounterVec,
    message_status: IntCounterVec,
    query_status: IntCounterVec,
}

impl MetricsConfig {
    pub fn register_into(registry: &MetricsRegistry) -> Self {
        Self {
            active_connections: registry.register(metric! {
                name: "mz_active_connections",
                help: "TODO",
                var_labels: ["internal"],
            }),
            connection_status: registry.register(metric! {
                name: "mz_connection_status",
                help: "TODO",
                var_labels: ["internal", "status"],
            }),
            message_status: registry.register(metric! {
                name: "mz_message_status",
                help: "TODO",
                var_labels: ["internal", "status"],
            }),
            query_status: registry.register(metric! {
                name: "mz_query_status",
                help: "TODO",
                var_labels: ["internal", "status"],
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Metrics {
    inner: MetricsConfig,
    internal: bool,
}

impl Metrics {
    pub fn new(inner: MetricsConfig, internal: bool) -> Self {
        let self_ = Self { inner, internal };

        // pre-initialize labels we are planning to use to ensure they are all
        // always emitted as time series
        self_.active_connections();
        self_.connection_status("success");
        self_.connection_status("error");
        self_.message_status("success");
        self_.message_status("error");
        self_.query_status("success");
        self_.query_status("error");

        self_
    }

    pub fn active_connections(&self) -> IntGauge {
        self.inner
            .active_connections
            .with_label_values(&[self.internal_label()])
    }

    pub fn connection_status(&self, status: &str) -> IntCounter {
        self.inner
            .connection_status
            .with_label_values(&[self.internal_label(), status])
    }

    pub fn message_status(&self, status: &str) -> IntCounter {
        self.inner
            .message_status
            .with_label_values(&[self.internal_label(), status])
    }

    pub fn query_status(&self, status: &str) -> IntCounter {
        self.inner
            .query_status
            .with_label_values(&[self.internal_label(), status])
    }

    fn internal_label(&self) -> &'static str {
        if self.internal {
            "true"
        } else {
            "false"
        }
    }
}
