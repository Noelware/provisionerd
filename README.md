# 🐻‍❄️🧊 `provisionerd` by Noelware, LLC.
> *Deploy VMs with code easily*

**provisionerd** is the core component on how Noelware functions as a provider. Since we don't use cloud resources and rely on bare-metal machines, we needed a way to provision VMs easily, and `provisionerd` does that. It is a Terraform-like concept, where it uses [HCL](https://github.com/hashicorp/hcl) files to provision VMs from different core components, it handles all the networking, disks, and more for you that is reusuable.

**provisionerd** comes in two binaries that you're able to use:

- `provisionerd` is the actual daemon that needs to be ran by the machine that hosts all VMs.
- `provctl` is the control management binary.

## License
**provisionerd** by Noelware, LLC. is released under the **Apache 2.0** License with love. Please read the [`LICENSE`](https://github.com/Noelware/provisionerd/blob/master/LICENSE) file for more information on what you can do with `provisionerd`.
