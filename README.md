# Dux : a scalable and cloud-friendly automation tool
![alt text](img/dux.png "Dux in the jungle"){width=100px}
# Our goal
Build a scalable automation / orchestration tool (something that runs a task on a remote host basically) ready to handle a massive amount of remote hosts in parallel. To increase the number of remote hosts you can handle in parallel, just increase the number of worker nodes or their capacity.

A worker node can be either a physical/virtual machine or a container.

# Usage

## All-in-one dux tool

`dux -t <tasklist.yaml> -l <hostlist.yaml> -k <SSH private key>`

with `tasklist.yaml`
~~~
- name: Install git
  steps:
    - !apt
      state: present
      package: git
    
    - !dnf
      state: present
      package: git
~~~
and `hostlist.yaml`
~~~
hosts:
  - 172.17.0.2
  - 172.17.0.3
  - 172.17.0.4
  - 172.17.0.5
  - 172.17.0.6
~~~

# Modules available
*(alphabetized)*
| Module | Description |
| ---      | ---      |
| `apt`   | Manage packages on Debian-like distributions |
| `dnf` | Manage packages on Fedora-like distributions (no difference with `yum`) |
| `ping`   | Test SSH connectivity with remote host |
| `yum` | Manage packages on Fedora-like distributions (no difference with `dnf`) |

## License
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
