Name:       duxtool
Version:    0.0.5
Release:    0
Summary:    Automation tool written in Rust
License:    Apache License, Version 2.0
URL:        https://gitlab.com/dux-tool/dux
Packager:   romzorus <romzorus591@gmail.com>


%description
A scalable and cloud-friendly automation / orchestration tool (all-in-one version) written in Rust.

%install
mkdir -p %{buildroot}/usr/bin
cp duxtool %{buildroot}/usr/bin/

%files
/usr/bin/duxtool
