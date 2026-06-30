# toml2nodeset

Turn TOML description of OPC-UA model to nodeset files.

This tool turns TOML description of OPC-UA ObjectTypes and their Variable
components into OPC-UA NodeSet2 files, via an UA Model Design intermediate
representation.

## Usage

### TOML design files

The source format is one TOML file for each ObjectType to be created. The JSON schema
from [this link](https://raw.githubusercontent.com/cailloumajor/toml2nodeset/refs/heads/main/schema/opc-object-type.schema.json)
can be used to help editing those files.

### Output

The tool will generate a Model Design file, with a name prefixed after the final
component of the provided directory, and will then use UA Model Compiler tool to
generate NodeSet files, with the same name prefix as the Model Design.

The OPC-UA namespace will be set to an URN, with namespace identifier part set as
the penultimate component of the provided directory path, and the namespace specific
part set the same as the ModelDesign prefix described above.

### Invocation

The tool is available through a [mise task](https://mise.jdx.dev/tasks/):

```ShellSession
mise run generate-nodeset <DIRECTORY>
````

The `DIRECTORY` argument is the path to the directory containing the TOML design
files, and where generated files will be written.
