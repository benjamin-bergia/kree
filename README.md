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
/home/test/projects/kustomize/examples/multibases/base/kustomization.yaml
/home/test/projects/kustomize/examples/multibases/base/pod.yaml
/home/test/projects/kustomize/examples/multibases/dev/kustomization.yaml
/home/test/projects/kustomize/examples/multibases/kustomization.yaml
/home/test/projects/kustomize/examples/multibases/production/kustomization.yaml
/home/test/projects/kustomize/examples/multibases/staging/kustomization.yaml
```
```bash
> kree -f json ../kustomize/examples/multibases | jq
[
  "/home/test/projects/kustomize/examples/multibases/base/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/base/pod.yaml",
  "/home/test/projects/kustomize/examples/multibases/dev/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/production/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/staging/kustomization.yaml"
]
```
```bash
> kree -f json ../kustomize/examples/multibases ../kustomize/examples/wordpress | jq
[
  "/home/test/projects/kustomize/examples/multibases/base/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/base/pod.yaml",
  "/home/test/projects/kustomize/examples/multibases/dev/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/production/kustomization.yaml",
  "/home/test/projects/kustomize/examples/multibases/staging/kustomization.yaml",
  "/home/test/projects/kustomize/examples/wordpress/kustomization.yaml",
  "/home/test/projects/kustomize/examples/wordpress/mysql/deployment.yaml",
  "/home/test/projects/kustomize/examples/wordpress/mysql/kustomization.yaml",
  "/home/test/projects/kustomize/examples/wordpress/mysql/secret.yaml",
  "/home/test/projects/kustomize/examples/wordpress/mysql/service.yaml",
  "/home/test/projects/kustomize/examples/wordpress/wordpress/deployment.yaml",
  "/home/test/projects/kustomize/examples/wordpress/wordpress/kustomization.yaml",
  "/home/test/projects/kustomize/examples/wordpress/wordpress/service.yaml"
]
```
Note: the result is sorted and doesn't follow the arguments order.
