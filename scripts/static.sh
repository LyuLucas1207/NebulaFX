#!/bin/sh
# Copyright 2024 RustFS Team
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# 已废弃：此脚本不再使用
# 前端已独立运行（rustfsconsole 项目），不再需要下载静态文件到 rustfs/static
# 如果需要构建前端并复制到 static 目录，请使用 scripts/build-console.sh

echo "⚠️  此脚本已废弃"
echo "   前端已独立运行，不再需要下载静态文件"
echo "   如需构建前端，请使用: ./scripts/build-console.sh"
exit 1
