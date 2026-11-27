# TypeScript

TypeScript (TS) is a high-level programming language that adds static typing with optional type annotations to JavaScript. It is designed for developing large applications. It transpiles to JavaScript. It is developed by Microsoft as free and open-source software released under the Apache License 2.0.

TypeScript may be used to develop JavaScript applications for both client-side and server-side execution (as with React.js, Node.js, Deno or Bun). Multiple options are available for transpiling. The default TypeScript Compiler can be used, or the Babel compiler can be invoked to convert TypeScript to JavaScript.

TypeScript supports definition files that can contain type information of existing JavaScript libraries, much like C++ header files can describe the structure of existing object files. This enables other programs to use the values defined in the files as if they were statically typed TypeScript entities. There are third-party header files for popular libraries such as jQuery, MongoDB, and D3.js. TypeScript headers for the Node.js library modules are also available, allowing development of Node.js programs within TypeScript.

Before TypeScript 7.0, the TypeScript compiler was written in TypeScript and compiled to JavaScript. In TypeScript 7.0, it was rewritten in Go. It is licensed under the Apache License 2.0. Anders Hejlsberg, lead architect of C# and creator of Delphi and Turbo Pascal, has worked on developing TypeScript.

## History

TypeScript was released to the public in October 2012, with version 0.8, after two years of internal development at Microsoft. Soon after the initial public release, Miguel de Icaza praised the language, but criticized the lack of mature integrated development environment (IDE) support apart from Microsoft Visual Studio, which was unavailable then on Linux and macOS. As of April 2021 there is support in other IDEs and text editors, including Emacs, Vim, WebStorm, Atom and Microsoft's own Visual Studio Code. TypeScript 0.9, released in 2013, added support for generics.

TypeScript 1.0 was released at Microsoft's Build developer conference in 2014. Visual Studio 2013 Update 2 provided built-in support for TypeScript. Further improvement were made in July 2014, when the development team announced a new TypeScript compiler, asserted to have a five-fold performance increase. Simultaneously, the source code, which was initially hosted on CodePlex, was moved to GitHub.

On 22 September 2016, TypeScript 2.0 was released, introducing several features, including the ability for programmers to optionally enforce null safety.

TypeScript 3.0 was released on 30 July 2018, bringing many language additions like tuples in rest parameters and spread expressions, rest parameters with tuple types, generic rest parameters and so on.

TypeScript 4.0 was released on 20 August 2020. While 4.0 did not introduce any breaking changes, it added language features such as Custom JSX Factories and Variadic Tuple Types.

TypeScript 5.0 was released on 16 March 2023 and included support for decorators.

On March 11, 2025, Anders Hejlsberg announced on the TypeScript blog that the team is working on a Go port of the TypeScript compiler to be released as TypeScript version 7.0 later this year. It is expected to feature a 10x speedup.

On December 2nd, 2025, Daniel Rosenwasser announced on the blog that TypeScript 6.0 will be the last release written in TypeScript itself, and TypeScript 7.0 will be the first Go-based release.

In GitHub's 2026 Octoverse report, TypeScript retained its position as the platform's most-used language by monthly contributors—ahead of Python and JavaScript —adding 3.62 million monthly contributors between September 2025 and September 2026 and serving as the primary language in 35% of newly created repositories with detected AI coding-agent activity.

## Design

TypeScript originated from the shortcomings of JavaScript for developing large-scale applications both at Microsoft and among their external customers. Challenges with dealing with complex JavaScript code led to demand for custom tooling to ease developing of components in the language.

Developers sought a solution that would not break compatibility with the ECMAScript (ES) standard and its ecosystem, so a compiler was developed to transform a superset of JavaScript with type annotations and classes (TypeScript files) back into vanilla ECMAScript 5 code. TypeScript classes were based on the then-proposed ECMAScript 6 class specification to make writing prototypal inheritance less verbose and error-prone, and type annotations enabled IntelliSense and improved tooling.

## Features

TypeScript adds the following syntax extensions to JavaScript:

Type signatures (annotations) and compile-time type checking

Type inference

Interfaces

Enumerated types

Generics

Namespaces

Tuples

Syntactically, TypeScript is very similar to JScript .NET, another Microsoft implementation of the ECMA-262 language standard that added support for static typing and classical object-oriented language features such as classes, inheritance, interfaces, and namespaces. Other inspirations include Java and C#.

## Compatibility with JavaScript

As TypeScript is simply a superset of JavaScript, existing JavaScript can be adapted to TypeScript while TypeScript can seamlessly integrate JavaScript. The compiler can target all ECMAScript versions 5 and above, transpiling modern features like classes and arrow functions to their older counterparts.

With TypeScript, it is possible to use existing JavaScript code, incorporate popular JavaScript libraries, and call TypeScript-generated code from other JavaScript. Type declarations for these libraries are usually provided with the source code but can be declared or installed separately if needed.

### Compiler

Prior to version 7, the TypeScript compiler, named tsc, was written in TypeScript and compiled into regular JavaScript, allowing it to run in any JavaScript engine (e.g. a browser) or as a Node.js package. Beginning with TypeScript 7.0, released on 8 July 2026, the compiler and language service were rewritten in Go as a native binary, which Microsoft reported delivers full-build speedups typically between 8 and 12 times over the JavaScript-based TypeScript 6.0 compiler; for example, type-checking the Visual Studio Code codebase fell from 125.7 seconds to 10.6 seconds under default settings. The native port began as an experimental preview distributed via the @typescript/native-preview npm package starting in 2025, which had accumulated more than 8.5 million weekly downloads before being merged into the standard typescript package for the 7.0 release. TypeScript 7.0 shipped without a stable programmatic compiler API, which Microsoft said would be introduced in a subsequent 7.1 release; in the interim, tooling that depends on the compiler's API, such as certain features of Vue, Angular, and Svelte, required continued use of the JavaScript-based compiler.

The compiler can target a given edition of ECMAScript (such as ECMAScript 5 for legacy browser compatibility), but by default compiles for the latest standards.

### Native compilation

In 2026, Vercel released scriptc, an open-source compiler that compiles TypeScript and JavaScript programs into self-contained native executables without requiring Node.js, V8, or another JavaScript engine in the resulting binary.

### IDE and editor support

Microsoft provides a plug-in for Visual Studio 2012 and WebMatrix, full integrated support in Visual Studio 2013, Visual Studio 2015, and basic text editor support for Emacs and Vim.

Visual Studio Code supports TypeScript in addition to several other languages, and offers features like debugging and intelligent code completion.

alm.tools is an open source cloud IDE for TypeScript built using TypeScript, ReactJS and TypeStyle.

JetBrains supports TypeScript with code completion, refactoring and debugging in its IDEs built on IntelliJ platform, such as PhpStorm 6, WebStorm 6, and IntelliJ IDEA, as well as their Visual Studio Add-in and extension, ReSharper 8.1.

Atom has a TypeScript plugin with support for code completion, navigation, formatting, and fast compilation.

The online Cloud9 IDE and Codenvy support TypeScript.

A plugin is available for the NetBeans IDE.

A plugin is available for the Eclipse IDE (version Kepler)

TypEcs is available for the Eclipse IDE.

The Cross Platform Cloud IDE Codeanywhere supports TypeScript.

Webclipse is an Eclipse plugin designed to develop TypeScript and Angular 2.

Angular IDE is a standalone IDE available via npm to develop TypeScript and Angular 2 applications, with integrated terminal support.

Tide –  TypeScript Interactive Development Environment for Emacs.

### Integration with build automation tools

Using plug-ins, TypeScript can be integrated with build automation tools, including Grunt (grunt-ts), Apache Maven (TypeScript Maven Plugin), Gulp (gulp-typescript) and Gradle (TypeScript Gradle Plugin).

### Linting tools

ESLint, a standard JavaScript linter, also provides support for TypeScript via the typescript-eslint plugin.

## Release history
