use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

include!("auth_test/prompt.rs");
include!("auth_test/types.rs");
include!("auth_test/run.rs");
include!("auth_test/probes.rs");
include!("auth_test/choice.rs");
