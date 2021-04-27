# Meta Extension Language (MEX)

MEX is a proof-of-concept tool to explore the idea of **stateful preprocessing**.

Stateful preprocessing can be likened to a super-preprocessor for meta-metaprogramming.

## Disclaimer

MEX is a research project. It is not yet intended for real-world use.

The research/white paper associated with MEX is currently in progress.


## What is MEX?

The goal of MEX is to preprocess code using custom directives. These directives can parse code, generating new directives.

The current example in `/bin/runner.rs` (ignore everything past line 109) demonstrates a program which registers C++ `struct` definitions and generates corresponding C-style constructors. 

MEX Example:
```C++
// See `/tests/example.mex.cpp`

@register
struct Example {
    int field1;
    char* field2;
};

// usage
auto example = @Example {
    .field2 = "meta",
    .field1 = 12,
};
```
In this example, the MEX directive `@register` generates the `@Example` MEX directive which converts its input into valid C++ code (writes output to `/tests/example.cpp`).

MEX offers a simple parsing library for extracting necessary data from source code, then saving this data throughout a code-generation step which outputs valid target-language code as defined by the user.

## Example Uses
Extend existing language such as:
- The above example
- Generate header files from function signatures in C/C++
- Add new syntax or functionality to a language
- ...

## Future Work
### **Extractive Grammars**
MEX operates by extracting data from existing source code, then generating new source code using that data.

A grammar would be able to take advantage of data extraction parsing methods such as:  
- "skip to next ';'"
- "parse word in reverse, keeping cursor in place, then advance cursor"
- ...

Such a grammar would be novel and remove the need for users to implement custom parsing logic.

### **Operate Using External Files**
MEX currently sticks everything in `/bin/runner.rs`. Ideally, the runner would:
1. Obtain the context, perhaps from a dynamic library
2. Obtain target `.mex` file directories
3. Follow instructions for how/where to save generated target language files

A potential file structure would be to have a `/.mex/` folder containing files `context.dll` and `targets.txt` for storing this information. Calling `mex` from the command line at a given directory would then operate using the `/.mex/` folder contents if present.