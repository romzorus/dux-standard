# Dux : automation tools written in Rust
<div align="center">
<img src="img/dux.png" width="25%">
</div>

# The goal
Instead of having one big automation tool (meaning configuration management or orchestration tool) trying to handle all scenarios (be scalable, performant, handle local and remote hosts through this protocol or this one, be compliant with this security standard and this one...), we prefer to build one flexible automation *engine* (the [duxcore](https://crates.io/crates/duxcore) crate) and make it as easy as possible to embed in a codebase already adapted to one's specific need.

This repository contains one implementation example : the standard version. It is the simplest use case : one binary on a host which applies configurations (task lists) to remote hosts.

<div align="center">
<img src="img/standard-illustration.png" width="70%">
</div>
Classic use case : a binary running on a Host and reaching out to other Hosts to apply some configuration (AKA Task list)

# Usage
~~~
dux -t <tasklist.yaml> -l <hostlist.yaml> -k <SSH private key> -u <username>
~~~

*with `tasklist.yaml`*
~~~
---
- name: Prerequisites
  steps:
    - name: 1. Test SSH connectivity
      ping:

    - name: 2. Upgrade the whole system
      with_sudo: true
      apt:
        upgrade: true

    - name: 3. Install git
      with_sudo: true
      apt:
        state: present
        package: "{{ packagename }}"
    
    - name: 4. Clean before clone
      command:
        content: rm -rf dux

    - name: 5. Clone a repository
      command:
       content: git clone https://github.com/romzorus/dux.git
~~~
*and `hostlist.yaml`*
~~~
vars:
  packagename: git

hosts:
  - 192.168.1.6
  - 192.168.1.85
~~~
**Output example**

~~~

    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚██╗██╔╝
    ██║  ██║██║   ██║ ╚███╔╝ 
    ██║  ██║██║   ██║ ██╔██╗ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝

Host 192.168.1.6 : Changed
Task : Prerequisites
┌───────────────────────────┬───────────────────────────────────────────────────────────┬────────────────────────────────────┐
│           Step            │                          Changes                          │Results                             │
├───────────────────────────┼───────────────────────────────────────────────────────────┼────────────────────────────────────┤
│1. Test SSH connectivity   │Check SSH connectivity with remote host                    │Success : Host reachable through SSH│
│2. Upgrade the whole system│Upgrade                                                    │Success : APT upgrade successful    │
│3. Install git             │Install - git                                              │Success : git install successful    │
│4. Clean before clone      │Run command : rm -rf dux                                   │Success : Command successfull       │
│5. Clone a repository      │Run command : git clone https://github.com/romzorus/dux.git│Success : Command successfull       │
└───────────────────────────┴───────────────────────────────────────────────────────────┴────────────────────────────────────┘

Host 192.168.1.85 : Changed
Task : Prerequisites
┌───────────────────────────┬───────────────────────────────────────────────────────────┬────────────────────────────────────┐
│           Step            │                          Changes                          │Results                             │
├───────────────────────────┼───────────────────────────────────────────────────────────┼────────────────────────────────────┤
│1. Test SSH connectivity   │Check SSH connectivity with remote host                    │Success : Host reachable through SSH│
│2. Upgrade the whole system│Upgrade                                                    │Success : APT upgrade successful    │
│3. Install git             │Package(s) already in expected state                       │None                                │
│4. Clean before clone      │Run command : rm -rf dux                                   │Success : Command successfull       │
│5. Clone a repository      │Run command : git clone https://github.com/romzorus/dux.git│Success : Command successfull       │
└───────────────────────────┴───────────────────────────────────────────────────────────┴────────────────────────────────────┘
~~~

# Have a remote host handled by Dux

## On controlled host
As for other automation tools, Dux needs an account to use on the controlled host. Let's create a `dux` user on the controlled host and give it some privileges :
```
# Create user (set password interactively)
sudo adduser dux

# Add user to sudo group
sudo usermod -aG sudo dux

# Create a sudoers file for this user
echo "dux ALL = (root) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/dux
```

## On controller host
The ideal is to have a SSH passwordless connection :
```
# Generate a SSH key (no passphrase for the example)
ssh-keygen -t ed25519 -f controller_key -N "" -q

# Have this key allowed on the controlled host
ssh-copy-id -i controller_key.pub dux@<controlled host address>
```

**After this, you can use `dux` like this from the controller node**

```dux -t <tasklist.yaml> -l <hostlist.yaml> -u dux -k <path to controller_key>```

## Want to contribute or just talk about this project ?
Open to suggestions, feedback, requests and any contribution ! Will gladly exchange ideas with you right [there](https://discord.com/invite/2gxAW7uzsx) !

# License
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
