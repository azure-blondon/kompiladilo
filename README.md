# kompiladilo
a compiler framework inspired by MLIR


kompiladilo is a framework that helps build Languages and Transformations between them.

The structure is designed to be generic and modular to take many Languages into account.

# But what is a Language?

It is a name and a set of possible Instructions.
This encompasses high-level languages, intermediate representations, low-level assembly languages, etc...

We call Module a sequence of Instructions that belong to a specific language.
(Modules containing Instructions of different Languages are not supported for now)

> Having Languages is fun, but it's better when they interact.

We can translate/compile a module from one language to another (and even from and to the same language).
This compilation step is called a Transformation.

Transformations can be chained easily into a pipeline.

Some Tranformations are not between Modules, but between Modules and some other medium (such as plain text or binary for example).
For these cases, it's possible to define Parsers and Emitters for any Language.
(for now, only String input and ouput are supported)