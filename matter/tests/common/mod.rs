/*
 *
 *    Copyright (c) 2020-2022 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

pub mod attributes;
pub mod commands;
pub mod echo_cluster;
pub mod handlers;
pub mod im_engine;

pub fn init_env_logger() {
    #[cfg(all(feature = "std", not(target_os = "espidf")))]
    {
        let _ = env_logger::try_init();
    }
}
