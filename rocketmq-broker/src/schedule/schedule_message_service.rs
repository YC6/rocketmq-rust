/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rocketmq_common::common::config_manager::ConfigManager;

#[derive(Default)]
pub struct ScheduleMessageService {}

impl ConfigManager for ScheduleMessageService {
    fn decode0(&mut self, _key: &[u8], _body: &[u8]) {
        todo!()
    }

    fn stop(&mut self) -> bool {
        todo!()
    }

    fn config_file_path(&mut self) -> &str {
        ""
    }

    fn encode(&mut self) -> String {
        todo!()
    }

    fn encode_pretty(&mut self, _pretty_format: bool) -> String {
        todo!()
    }

    fn decode(&mut self, _json_string: &str) {
        todo!()
    }
}