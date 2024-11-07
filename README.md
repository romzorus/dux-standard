# Dux : standard implementation
<div align="center">
<img src="img/dux.png" width="25%">
</div>

This repository contains one example of implementation of the [duxcore](https://crates.io/crates/duxcore) crate : the standard version. It is the simplest use case : one binary on a host which applies configurations (task lists) to remote hosts.

<div align="center">
<img src="img/standard-illustration.png" width="60%">
</div>


# Usage
~~~shell
dux -t <tasklist.yaml> -l <hostlist.yaml> -k <SSH private key> -u <username>
~~~

*with `tasklist.yaml`*
~~~yaml
- name: Let's install a web server !
  steps:
    - name: First, we test the connectivity and authentication with the host.
      ping:
      
    - name: Then we can install the package...
      with_sudo: true
      apt:
        package: '{{ package_name }}'
        state: present
        
    - name: ... and start & enable the service.
      with_sudo: true
      service:
        name: '{{ service_name }}'
        state: started
        enabled: true
~~~
*and `hostlist.yaml`*
~~~yaml
vars:
  package_name: apache2
  service_name: apache2

hosts:
  - 10.20.0.203
  - 10.20.0.204
~~~
**Output example**

~~~json
{
  "jobs": [
    {
      "host": "10.20.0.203",
      "timestamp_start": "2024-11-07T22:53:52.781400539+00:00",
      "timestamp_end": "2024-11-07T22:54:06.610757092+00:00",
      "final_status": "ApplySuccesful",
      "tasks": [
        {
          "name": "Let's install a web server !",
          "steps": [
            {
              "name": "First, we test the connectivity and authentication with the host.",
              "expected_state": {
                "ping": {}
              },
              "status": "ApplySuccessful"
            },
            {
              "name": "Then we can install the package...",
              "expected_state": {
                "apt": {
                  "state": "present",
                  "package": "apache2"
                }
              },
              "status": "ApplySuccessful"
            },
            {
              "name": "... and start & enable the service.",
              "expected_state": {
                "service": {
                  "name": "apache2",
                  "state": "started",
                  "enabled": true
                }
              },
              "status": "ApplySuccessful"
            }
          ]
        }
      ]
    },
    {
      "host": "10.20.0.204",
      "timestamp_start": "2024-11-07T22:53:52.742561538+00:00",
      "timestamp_end": "2024-11-07T22:54:04.799505739+00:00",
      "final_status": "ApplySuccesful",
      "tasks": [
        {
          "name": "Let's install a web server !",
          "steps": [
            {
              "name": "First, we test the connectivity and authentication with the host.",
              "expected_state": {
                "ping": {}
              },
              "status": "ApplySuccessful"
            },
            {
              "name": "Then we can install the package...",
              "expected_state": {
                "apt": {
                  "state": "present",
                  "package": "apache2"
                }
              },
              "status": "ApplySuccessful"
            },
            {
              "name": "... and start & enable the service.",
              "expected_state": {
                "service": {
                  "name": "apache2",
                  "state": "started",
                  "enabled": true
                }
              },
              "status": "ApplySuccessful"
            }
          ]
        }
      ]
    }
  ]
}

~~~

# Have a remote host handled by Dux

## On controlled host
As for other automation tools, Dux needs an account to use on the controlled host. Let's create a `dux` user on the controlled host and give it some privileges :
```shell
# Create user (set password interactively)
sudo adduser dux

# Add user to sudo group
sudo usermod -aG sudo dux

# Create a sudoers file for this user
echo "dux ALL = (root) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/dux
```

## On controller host
The ideal is to have a SSH passwordless connection :
```shell
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

