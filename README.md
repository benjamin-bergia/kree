# Kree (Kustomization Tree)

Kree let you list all the resources included by a specific kustomizaition file.

## Limitations

  - Remote resources are currently ignored and will not appear in the ouput.

## Example

```bash
> kree --help
Usage: kree [OPTIONS] <PATH>

Arguments:
  <PATH>
          Path to the kustomization file or directory

Options:
  -f, --format <FORMAT>
          Output format
          
          [default: text]

          Possible values:
          - text: One path per line
          - json: JSON

  -h, --help
          Print help information (use `-h` for a summary)
```
```bash
> kree ../kustomize/examples/multibases
../kustomize/examples/multibases/base/kustomization.yaml
../kustomize/examples/multibases/base/pod.yaml
../kustomize/examples/multibases/dev/kustomization.yaml
../kustomize/examples/multibases/kustomization.yaml
../kustomize/examples/multibases/production/kustomization.yaml
../kustomize/examples/multibases/staging/kustomization.yaml
```
```bash
> kree -f json ../kustomize/examples/multibases | jq
[
  "../kustomize/examples/multibases/base/kustomization.yaml",
  "../kustomize/examples/multibases/base/pod.yaml",
  "../kustomize/examples/multibases/dev/kustomization.yaml",
  "../kustomize/examples/multibases/kustomization.yaml",
  "../kustomize/examples/multibases/production/kustomization.yaml",
  "../kustomize/examples/multibases/staging/kustomization.yaml"
]
```
```bash
> kree -f json ../kustomize/examples/multibases ../kustomize/examples/wordpress | jq
[
  "../kustomize/examples/multibases/base/kustomization.yaml",
  "../kustomize/examples/multibases/base/pod.yaml",
  "../kustomize/examples/multibases/dev/kustomization.yaml",
  "../kustomize/examples/multibases/kustomization.yaml",
  "../kustomize/examples/multibases/production/kustomization.yaml",
  "../kustomize/examples/multibases/staging/kustomization.yaml",
  "../kustomize/examples/wordpress/kustomization.yaml",
  "../kustomize/examples/wordpress/mysql/deployment.yaml",
  "../kustomize/examples/wordpress/mysql/kustomization.yaml",
  "../kustomize/examples/wordpress/mysql/secret.yaml",
  "../kustomize/examples/wordpress/mysql/service.yaml",
  "../kustomize/examples/wordpress/wordpress/deployment.yaml",
  "../kustomize/examples/wordpress/wordpress/kustomization.yaml",
  "../kustomize/examples/wordpress/wordpress/service.yaml"
]
```
Note: the result is sorted and doesn't follow the arguments order.
