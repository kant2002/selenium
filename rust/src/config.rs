// Licensed to the Software Freedom Conservancy (SFC) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SFC licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::config::OS::{LINUX, MACOS, WINDOWS};
use crate::{ARCH_AMD64, ARCH_ARM64, ARCH_X86, TTL_BROWSERS_SEC, TTL_DRIVERS_SEC, WMIC_COMMAND_OS};

use crate::{format_one_arg, run_shell_command, REQUEST_TIMEOUT_SEC, UNAME_COMMAND};
use std::env::consts::OS;

pub struct ManagerConfig {
    pub browser_version: String,
    pub driver_version: String,
    pub os: String,
    pub arch: String,
    pub browser_path: String,
    pub proxy: String,
    pub timeout: u64,
    pub browser_ttl: u64,
    pub driver_ttl: u64,
}

impl ManagerConfig {
    pub fn default() -> ManagerConfig {
        let self_os = OS;
        let self_arch = if WINDOWS.is(self_os) {
            if run_shell_command(self_os, WMIC_COMMAND_OS.to_string())
                .unwrap_or_default()
                .contains("32")
            {
                ARCH_X86.to_string()
            } else {
                ARCH_AMD64.to_string()
            }
        } else {
            let uname_a = format_one_arg(UNAME_COMMAND, "a");
            if run_shell_command(self_os, uname_a)
                .unwrap_or_default()
                .to_ascii_lowercase()
                .contains(ARCH_ARM64)
            {
                ARCH_ARM64.to_string()
            } else {
                let uname_m = format_one_arg(UNAME_COMMAND, "m");
                run_shell_command(self_os, uname_m).unwrap_or_default()
            }
        };
        ManagerConfig {
            browser_version: "".to_string(),
            driver_version: "".to_string(),
            os: self_os.to_string(),
            arch: self_arch,
            browser_path: "".to_string(),
            proxy: "".to_string(),
            timeout: REQUEST_TIMEOUT_SEC,
            browser_ttl: TTL_BROWSERS_SEC,
            driver_ttl: TTL_DRIVERS_SEC,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn clone(config: &ManagerConfig) -> ManagerConfig {
        ManagerConfig {
            browser_version: config.browser_version.as_str().to_string(),
            driver_version: config.driver_version.as_str().to_string(),
            os: config.os.as_str().to_string(),
            arch: config.arch.as_str().to_string(),
            browser_path: config.browser_path.as_str().to_string(),
            proxy: config.proxy.as_str().to_string(),
            timeout: config.timeout,
            browser_ttl: config.browser_ttl,
            driver_ttl: config.driver_ttl,
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum OS {
    WINDOWS,
    MACOS,
    LINUX,
}

impl OS {
    pub fn to_str(&self) -> &str {
        match self {
            WINDOWS => "windows",
            MACOS => "macos",
            LINUX => "linux",
        }
    }

    pub fn is(&self, os: &str) -> bool {
        self.to_str().eq_ignore_ascii_case(os)
    }
}

pub fn str_to_os(os: &str) -> OS {
    if WINDOWS.is(os) {
        WINDOWS
    } else if MACOS.is(os) {
        MACOS
    } else {
        LINUX
    }
}

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum ARCH {
    X32,
    X64,
    ARM64,
}

impl ARCH {
    pub fn to_str_vector(&self) -> Vec<&str> {
        match self {
            ARCH::X32 => vec![ARCH_X86, "i386"],
            ARCH::X64 => vec![ARCH_AMD64, "x86_64", "x64", "i686", "ia64"],
            ARCH::ARM64 => vec![ARCH_ARM64, "aarch64", "arm"],
        }
    }

    pub fn is(&self, arch: &str) -> bool {
        self.to_str_vector()
            .contains(&arch.to_ascii_lowercase().as_str())
    }
}
