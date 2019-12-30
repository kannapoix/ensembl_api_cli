# Ensemble API CLI
This is cli for calling [Ensembl API](https://rest.ensembl.org/).  
This project is currently at the experimental stage.  
Not all endpoints are supported.  
Only following endpoints and data types are callable.  

## Callable endpoints
|  Endpoint  |  Options  |
| ---- | ---- |
|  GET /sequence/id  |  object_type=transcript, content-type=text/xfasta |

# Download
Binaries are available at the [release paga](https://github.com/kannapoix/ensembl_api_cli/releases) of Github.  

# How to use
## From ID
This will get transctipt as fasta format.
```
$ ensemble_api_cli --id <ID>
```

## From file
You can specify a file including IDs in each line.  
Results are outputted at ./result directory.
```
$ ensembl_api_cli --directory <Path to directory>
```

## Other options
Help option can be put for any command or subcommand.
```
$ ensembl_api_cli --help
$ ensembl_api_cli swquence id --help
```
