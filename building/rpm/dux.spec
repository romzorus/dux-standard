Name:       dux
Version:    PLACEHOLDER_FOR_VERSION
Release:    0
Summary:    Automation tool written in Rust
License:    Apache License, Version 2.0
URL:        https://gitlab.com/dux-tool/dux
Packager:   romzorus <romzorus591@gmail.com>


%description
A scalable and cloud-friendly automation / orchestration tool (all-in-one version) written in Rust.

%install
mkdir -p %{buildroot}/usr/bin
cp dux %{buildroot}/usr/bin/
mkdir -p %{buildroot}/etc/dux
cp dux.conf %{buildroot}/etc/dux/

%files
/usr/bin/dux
/etc/dux/dux.conf
