% A Practical Intro to Macros in Rust 1.0

<style type="text/css">
    /**
     * Copyright 2013 The Rust Project Developers. See the COPYRIGHT
     * file at the top-level directory of this distribution and at
     * http://rust-lang.org/COPYRIGHT.
     * With elements taken from Bootstrap v3.0.2 (MIT licensed).
     *
     * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
     * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
     * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
     * option. This file may not be copied, modified, or distributed
     * except according to those terms.
     */
    *:not(body) {
      -webkit-box-sizing: border-box;
         -moz-box-sizing: border-box;
              box-sizing: border-box;
    }

    /* General structure */

    body {
        margin: 0 auto;
        padding: 0 15px;
        font-family: "Source Serif Pro", "Helvetica Neue", Helvetica, Arial, sans-serif;
        font-size: 18px;
        color: #333;
        line-height: 1.428571429;
    }
    @media (min-width: 768px) {
        body {
            max-width: 750px;
        }
    }

    h1, h2, h3, h4, h5, h6, nav, #versioninfo {
        font-family: "Fira Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
    }
    h1, h2, h3, h4, h5, h6 {
        color: black;
        font-weight: 400;
        line-height: 1.1;
    }
    h1, h2, h3 {
        margin-top: 20px;
        margin-bottom: 15px;
    }
    h1 {
        margin-bottom: 20px;
    }
    h4, h5, h6 {
        margin-top: 12px;
        margin-bottom: 10px;
        padding: 5px 10px;
    }
    h5, h6 {
        text-decoration: underline;
    }

    h1 {
        font-size: 28px;
        font-weight: 500;
        padding: .1em .4em;
        border-bottom: 2px solid #ddd;
    }
    h1.title {
        line-height: 1.5em;
    }
    h2 {
        font-size: 26px;
        padding: .2em .5em;
        border-bottom: 1px solid #ddd;
    }
    h3 {
        font-size: 24px;
        padding: .2em .7em;
        border-bottom: 1px solid #DDE8FC;
    }
    h4 {
        font-size: 22px;
    }
    h5 {
        font-size: 20px;
    }
    h6 {
        font-size: 18px;
    }
    @media (min-width: 992px) {
        h1 {
            font-size: 36px;
        }
        h2 {
            font-size: 30px;
        }
        h3 {
            font-size: 26px;
        }
    }

    nav {
        column-count: 2;
        -moz-column-count: 2;
        -webkit-column-count: 2;
        font-size: 15px;
        margin: 0 0 1em 0;
    }
    p {
        margin: 0 0 1em 0;
    }

    strong {
        font-weight: bold;
    }

    em {
        font-style: italic;
    }

    footer {
        border-top: 1px solid #ddd;
        font-size: 14.3px;
        font-style: italic;
        padding-top: 5px;
        margin-top: 3em;
        margin-bottom: 1em;
    }

    /* Links layout */

    a {
        text-decoration: none;
        color: #428BCA;
        background: transparent;
    }
    a:hover, a:focus {
        color: #2A6496;
        text-decoration: underline;
    }
    a:focus {
        outline: thin dotted #333;
        outline: 5px auto -webkit-focus-ring-color;
        outline-offset: -2px;
    }
    a:hover, a:active {
        outline: 0;
    }

    h1 a:link, h1 a:visited, h2 a:link, h2 a:visited,
    h3 a:link, h3 a:visited, h4 a:link, h4 a:visited,
    h5 a:link, h5 a:visited {color: black;}
    h1 a:hover, h2 a:hover, h3 a:hover, h4 a:hover,
    h5 a:hover {text-decoration: none;}

    /* Code */

    pre, code {
        font-family: "Source Code Pro", Menlo, Monaco, Consolas, "DejaVu Sans Mono", monospace;
    }
    pre {
        border-left: 2px solid #eee;
        white-space: pre-wrap;
        padding: 14px;
        padding-right: 0;
        margin: 20px 0;
        font-size: 13px;
        word-break: break-all;
        word-wrap: break-word;
    }
    code {
        padding: 0 2px;
        color: #8D1A38;
        white-space: pre-wrap;
    }
    pre code {
        padding: 0;
        font-size: inherit;
        color: inherit;
    }

    a > code {
        color: #428BCA;
    }

    /* Code highlighting */
    pre.rust .kw { color: #8959A8; }
    pre.rust .kw-2, pre.rust .prelude-ty { color: #4271AE; }
    pre.rust .number, pre.rust .string { color: #718C00; }
    pre.rust .self, pre.rust .boolval, pre.rust .prelude-val,
    pre.rust .attribute, pre.rust .attribute .ident { color: #C82829; }
    pre.rust .comment { color: #8E908C; }
    pre.rust .doccomment { color: #4D4D4C; }
    pre.rust .macro, pre.rust .macro-nonterminal { color: #3E999F; }
    pre.rust .lifetime { color: #B76514; }

    /* The rest */

    #versioninfo {
        text-align: center;
        margin: 0.5em;
        font-size: 1.1em;
    }
    @media (min-width: 992px) {
        #versioninfo {
            font-size: 0.8em;
            position: fixed;
            bottom: 0px;
            right: 0px;
        }
        .white-sticker {
            background-color: #fff;
            margin: 2px;
            padding: 0 2px;
            border-radius: .2em;
        }
    }
    #versioninfo a.hash {
        color: gray;
        font-size: 80%;
    }

    blockquote {
        color: #000;
        margin: 20px 0;
        padding: 15px 20px;
        background-color: #f2f7f9;
        border-top: .1em solid #e5eef2;
        border-bottom: .1em solid #e5eef2;
    }
    blockquote p {
        font-size: 17px;
        font-weight: 300;
        line-height: 1.4;
    }
    blockquote p:last-child {
        margin-bottom: 0;
    }

    ul, ol {
        padding-left: 25px;
    }
    ul ul, ol ul, ul ol, ol ol {
        margin-bottom: 0;
    }
    dl {
        margin-bottom: 20px;
    }
    dd {
        margin-left: 0;
    }

    nav ul {
        list-style-type: none;
        margin: 0;
        padding-left: 0px;
    }

    /* Only display one level of hierarchy in the TOC */
    nav ul ul {
        display: none;
    }

    sub,
    sup {
        font-size: 75%;
        line-height: 0;
        position: relative;
    }

    hr {
        margin-top: 20px;
        margin-bottom: 20px;
        border: 0;
        border-top: 1px solid #eeeeee;
    }

    table {
        border-collapse: collapse;
        border-spacing: 0;
    }

    table tr.odd {
        background: #eee;
    }

    table td,
    table th {
        border: 1px solid #ddd;
        padding: 5px;
    }

    /* Code snippets */

    .rusttest { display: none; }
    pre.rust { position: relative; }
    .test-arrow {
        display: inline-block;
        position: absolute;
        top: 0;
        right: 10px;
        font-size: 150%;
        -webkit-transform: scaleX(-1);
        transform: scaleX(-1);
    }

    .unstable-feature {
        border: 2px solid red;
        padding: 5px;
    }

    @media (min-width: 1170px) {
        pre {
            font-size: 15px;
        }
    }

    @media print {
        * {
            text-shadow: none !important;
            color: #000 !important;
            background: transparent !important;
            box-shadow: none !important;
        }
        a, a:visited {
            text-decoration: underline;
        }
        p a[href]:after {
            content: " (" attr(href) ")";
        }
        footer a[href]:after {
            content: "";
        }
        a[href^="javascript:"]:after, a[href^="#"]:after {
            content: "";
        }
        pre, blockquote {
            border: 1px solid #999;
            page-break-inside: avoid;
        }
        @page {
            margin: 2cm .5cm;
        }
        h1:not(.title), h2, h3 {
            border-bottom: 0px none;
        }
        p, h2, h3 {
            orphans: 3;
            widows: 3;
        }
        h2, h3 {
            page-break-after: avoid;
        }
        table {
            border-collapse: collapse !important;
        }
        table td, table th {
            background-color: #fff !important;
        }
    }

    #keyword-table-marker + table thead { display: none; }
    #keyword-table-marker + table td { border: none; }
    #keyword-table-marker + table {
        margin-left: 2em;
        margin-bottom: 1em;
    }
</style>

<style type="text/css">
    /* Customisations. */

    .small-code code {
        font-size: 60%;
    }

    table pre.rust {
        margin: 0;
        border: 0;
    }

    table.parse-table tbody > tr > td:nth-child(1) > code:nth-of-type(2) {
        color: black;
        margin-top: -0.7em;
        margin-bottom: -0.6em;
    }

    table.parse-table tbody > tr > td:nth-child(1) > code {
        display: block;
    }

    table.parse-table tbody > tr > td:nth-child(2) > code {
        display: block;
    }

    .katex {
        font: 400 1.21em/1.2 KaTeX_Main;
        white-space: nowrap;
        font-family: "Cambria Math", "Cambria", serif;
    }

    .katex .vlist > span > {
        display: inline-block;
    }

    .mathit {
        font-style: italic;
    }

    .katex .reset-textstyle.scriptstyle {
        font-size: 0.7em;
    }

    .katex .reset-textstyle.textstyle {
        font-size: 1em;
    }

    .katex .textstyle > .mord + .mrel {
        margin-left: 0.27778em;
    }

    .katex .textstyle > .mrel + .minner, .katex .textstyle > .mrel + .mop, .katex .textstyle > .mrel + .mopen, .katex .textstyle > .mrel + .mord {
        margin-left: 0.27778em;
    }

    .katex .textstyle > .mclose + .minner, .katex .textstyle > .minner + .mop, .katex .textstyle > .minner + .mord, .katex .textstyle > .mpunct + .mclose, .katex .textstyle > .mpunct + .minner, .katex .textstyle > .mpunct + .mop, .katex .textstyle > .mpunct + .mopen, .katex .textstyle > .mpunct + .mord, .katex .textstyle > .mpunct + .mpunct, .katex .textstyle > .mpunct + .mrel {
        margin-left: 0.16667em;
    }

    .katex .textstyle > .mord + .mbin {
        margin-left: 0.22222em;
    }

    .katex .textstyle > .mbin + .minner, .katex .textstyle > .mbin + .mop, .katex .textstyle > .mbin + .mopen, .katex .textstyle > .mbin + .mord {
        margin-left: 0.22222em;
    }
</style>

One feature of Rust that I'm rather enamoured with is its macro system.  Known as "macro by example" (MBE) or sometimes referred to as `macro_rules`, it provides you with an impressive amount of power whilst not allowing for the sort of evil tricks you can pull in C/C++.

The purpose of this article is to go through the *process* by which I wrote one *particular* macro, in the hopes that showing off the process itself will prove useful.  As a result, I will try to ensure that there are complete, unabridged code examples at reasonable intervals along the way for you to copy out and play with if you so desire.  If you don't want to, the final product will also be available as a finished library.

If this isn't how you like to learn, or you simply want more information, there is also the [Macros chapter of the Rust Book](http://doc.rust-lang.org/stable/book/macros.html).

To set the stage, we will be going through the design and construction of a macro for quickly defining recurrence relations as iterators.  Ideally, the following snippet of code should print the first ten numbers in the Fibonacci sequence:

```ignore
let fib = recurrence![ a[n]: u64 = 0, 1 ... a[n-1] + a[n-2] ];

for e in fib.take(10) { println!("{}", e) }
```

# A Little Context

> **Note**: don't panic!  What follows is the only time D or math will be talked about.  You can quite safely skip this section if you just want to get to the meat of the article.

This particular macro is inspired by a function in the D standard library.  Specifically, [`std.range.recurrence`](http://dlang.org/phobos/std_range.html#recurrence) is a templated function which returns an iterator (called a "range" in D parlance) that yields successive elements of a recurrence relation.  I wanted to see how close to the original D template I could get using Rust macros.

If you aren't familiar, a recurrence relation is a sequence where each value is defined in terms of one or more *previous* values, with one or more initial values to get the whole thing started.  For example, the [Fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_number) can be defined by the relation:

<div class="katex" style="font-size: 100%; text-align: center;">
    <span class="katex"><span class="katex-inner"><span style="height: 0.68333em;" class="strut"></span><span style="height: 0.891661em; vertical-align: -0.208331em;" class="strut bottom"></span><span class="base textstyle uncramped"><span class="reset-textstyle displaystyle textstyle uncramped"><span class="mord displaystyle textstyle uncramped"><span class="mord"><span class="mord mathit" style="margin-right: 0.13889em;">F</span><span class="vlist"><span style="top: 0.15em; margin-right: 0.05em; margin-left: -0.13889em;" class=""><span class="fontsize-ensurer reset-size5 size5"><span style="font-size: 0em;" class="">​</span></span><span class="reset-textstyle scriptstyle cramped"><span class="mord mathit">n</span></span></span><span class="baseline-fix"><span class="fontsize-ensurer reset-size5 size5"><span style="font-size: 0em;" class="">​</span></span>​</span></span></span><span class="mrel">=</span><span class="mord">0</span><span class="mpunct">,</span><span class="mord">1</span><span class="mpunct">,</span><span class="mpunct">…</span><span class="mpunct">,</span><span class="mord"><span class="mord mathit" style="margin-right: 0.13889em;">F</span><span class="vlist"><span style="top: 0.15em; margin-right: 0.05em; margin-left: -0.13889em;" class=""><span class="fontsize-ensurer reset-size5 size5"><span style="font-size: 0em;" class="">​</span></span><span class="reset-textstyle scriptstyle cramped"><span class="mord scriptstyle cramped"><span class="mord mathit">n</span><span class="mbin">−</span><span class="mord">1</span></span></span></span><span class="baseline-fix"><span class="fontsize-ensurer reset-size5 size5"><span style="font-size: 0em;" class="">​</span></span>​</span></span></span><span class="mbin">+</span><span class="mord"><span class="mord mathit" style="margin-right: 0.13889em;">F</span><span class="vlist"><span style="top: 0.15em; margin-right: 0.05em; margin-left: -0.13889em;" class=""><span class="fontsize-ensurer reset-size5 size5"><span style="font-size: 0em;" class="">​</span></span><span class="reset-textstyle scriptstyle cramped"><span class="mord scriptstyle cramped"><span class="mord mathit">n</span><span class="mbin">+</span><span class="mord">2</span></span></span></span><span class="baseline-fix"><span class="fontsize-ensurer reset-size5 size5"><span style="font-size: 0em;" class="">​</span></span>​</span></span></span></span></span></span></span></span>
</div>

Thus, the first two numbers in the sequence are 0 and 1, with the third being <em>F<sub>0</sub></em> + <em>F<sub>1</sub></em> = 0 + 1 = 1, the fourth <em>F<sub>1</sub></em> + <em>F<sub>2</sub></em> = 1 + 1 = 2, and so on forever.

Now, *because* such a sequence can go on forever, that makes defining a `fibonacci` function a little tricky, since you obviously don't want to try returning a complete vector.  What you *want* is to return something which will lazily compute elements of the sequence as needed.

In Rust, that means producing an `Iterator`.  This is not especially *hard*, but there is a fair amount of boilerplate involved: you need to define a custom type, work out what state needs to be stored in it, then implement the `Iterator` trait for it.

However, recurrence relations are simple enough that almost all of these details can be abstracted out with a little macro-based code generation.

So, with all that having been said, let's get started.

# Macro Parsing and Expansion

Before going into the construction of the `recurrence!` macro, it may be helpful to understand how Rust parses macros.  If you're already comfortable with this, feel free to skip this section.

A macro invocation in Rust is, in contrast to something like C, not a wholly separate pass over the source code.  Macro invocations are actually a normal part of the compiler's AST representation.  This means that invocations can *only* appear in positions where they're explicitly supported.  Currently, they can appear in place of items, methods, statements, expressions, and patterns.  Note that, as a consequence of this, there are some things you can't do with macros, such as have one which expands to the identifier for a function declaration.

However, the status of macro invocations as first-class members of the AST means that the Rust parser has to be able to parse them into something sensible, even when they use syntax that Rust itself doesn't support.  The way this is done is by parsing the contents of an invocation into "token trees".  If we take the `fib` example above, given the invocation:

```ignore
recurrence![ a[n]: u64 = 1, 1 ... a[n-1] + a[n-2] ]
```

the invocation arguments stored in the AST look something like:

```text
[ `a` `[ ]` `:` `u64` `=` `1` `,` `1` `...` `a` `[ ]` `+` `a` `[ ]` ]
        ^                                         ^             |
     [ `n` ]                               [ `n` `-` `1` ]      ^
                                                         [ `n` `-` `2` ]
```

Sequences enclosed by parentheses, braces, or brackets become a single logical "token tree" node.  This is how the parser keeps track of how deep into a macro invocation it is, and when to stop parsing it.  This is *also* why your macro syntax must have balanced parens, braces, and brackets.

When it comes time to expand a macro invocation, the compiler feeds the parsed token trees into the macro, which must expand to a new sequence of token trees which can be parsed as an AST node that matches the invocation's position.  In other words, if you have a macro invocation in expression position, the token trees which it expands to *must* be parseable as an expression.

This means that not only is *where* you can use a macro restricted, you also *cannot* have a macro which expands to something that isn't a complete, valid Rust construct.  This has the nice property that you cannot, for example, forget a closing paren or brace in a macro.

# Construction

Usually, when working on a new macro, the first thing I do is decide what the macro invocation should look like.  In this specific case, my first attempt looked like this:

```ignore
let fib = recurrence![a[n] = 1, 1, ..., a[n-1] + a[n-2]];

for e in fib.take(10) { println!("{}", e) }
```

From that, we can take a stab at how the macro should be defined, even if we aren't sure of the actual expansion.  This is useful because if you can't figure out how to parse the input syntax, then *maybe* you need to change it.

```rust
macro_rules! recurrence {
    ( a[n] = $($inits:expr),+ , ... , $recur:expr ) => { /* ... */ };
}
# fn main() {}
```

Assuming you aren't familiar with the syntax, allow me to elucidate.  This is defining a macro using the `macro_rules` machinery (there is one other way to define macros, but we'll come back to that) called `recurrence`.  This macro has a single parsing rule.  That rule says the input to the macro must match:

- the literal token sequence `a [ n ] =`,
- a repeating (the `$( ... )`) sequence, using `,` as a separator, and one or more (`+`) repeats of:
    - a valid *expression* captured into the variable `inits` (`$inits:expr`)
- the literal token sequence `, ... ,`,
- a valid *expression* captured into the variable `recur` (`$recur:expr`).

Finally, the rule says that *if* the input matches this rule, then the macro invocation should be replaced by the token sequence `/* ... */`.

It's worth noting that `inits`, as implied by the name, actually contains *all* the expressions that match in this position, not just the first or last.  What's more, it captures them *as a sequence* as opposed to, say, irreversibly pasting them all together.  Also note that you can do "zero or more" with a repetition by using `*` instead of `+`.  There is no support for "zero or one" or more specific numbers of repetitions.

As an exercise, let's take the proposed input and feed it through the rule, to see how it is processed.  The "Position" column will show which part of the syntax pattern needs to be matched against next, denoted by a "⌂".  Note that in some cases, there might be more than one possible "next" element to match against.  "Input" will contain all of the tokens that have *not* been consumed yet.  `inits` and `recur` will contain the contents of those bindings.

<table class="parse-table">
    <thead>
        <tr>
            <th>Position</th>
            <th>Input</th>
            <th><code>inits</code></th>
            <th><code>recur</code></th>
        </tr>
    </thead>
    <tbody class="small-code">
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>⌂</code></td>
            <td><code>a[n] = 1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code> ⌂</code></td>
            <td><code>[n] = 1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>  ⌂</code></td>
            <td><code>n] = 1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>   ⌂</code></td>
            <td><code>] = 1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>     ⌂</code></td>
            <td><code>= 1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>       ⌂</code></td>
            <td><code>1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>         ⌂</code></td>
            <td><code>1, 1, ..., a[n-1] + a[n-2]</code></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>                     ⌂  ⌂</code></td>
            <td><code>, 1, ..., a[n-1] + a[n-2]</code></td>
            <td><code>1</code></td>
            <td></td>
        </tr>
        <tr>
            <td colspan="4" style="font-size:.7em;">

<em>Note</em>: there are two ⌂ here, because a comma could mean <em>either</em> another element in the repetition, <em>or</em> the comma <em>after</em> the repetition.  The macro system will keep track of both possibilities, until it is able to decide which one to follow.

            </td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>         ⌂                ⌂</code></td>
            <td><code>1, ..., a[n-1] + a[n-2]</code></td>
            <td><code>1</code></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>                     ⌂  ⌂ <s>⌂</s></code></td>
            <td><code>, ..., a[n-1] + a[n-2]</code></td>
            <td><code>1</code>, <code>1</code></td>
            <td></td>
        </tr>
        <tr>
            <td colspan="4" style="font-size:.7em;">

<em>Note</em>: the third, crossed-out marker indicates that the macro system has, as a consequence of the last token consumed, eliminated one of the previous possible branches.

            </td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>         ⌂                ⌂</code></td>
            <td><code>..., a[n-1] + a[n-2]</code></td>
            <td><code>1</code>, <code>1</code></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>         <s>⌂</s>                    ⌂</code></td>
            <td><code>, a[n-1] + a[n-2]</code></td>
            <td><code>1</code>, <code>1</code></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>                                ⌂</code></td>
            <td><code>a[n-1] + a[n-2]</code></td>
            <td><code>1</code>, <code>1</code></td>
            <td></td>
        </tr>
        <tr>
            <td><code>a[n] = $($inits:expr),+ , ... , $recur:expr</code>
                <code>                                           ⌂</code></td>
            <td></td>
            <td><code>1</code>, <code>1</code></td>
            <td><code>a[n-1] + a[n-2]</code></td>
        </tr>
        <tr>
            <td colspan="4" style="font-size:.7em;">

<em>Note</em>: this particular step should make it clear that a binding like <tt>$recur:expr</tt> will consume an <em>entire expression</em>, using the compiler's knowledge of what constitutes a valid expression.  As will be noted later, you can do this for other language constructs, too.

            </td>
        </tr>
    </tbody>
</table>

<p></p>

The key take-away from this is that the macro system will *try* to incrementally match the tokens provided as input to the macro against the provided rules.  We'll come back to the "try" part.

Now, let's begin writing the final, fully expanded form.  For this expansion, I was looking for something like:

```ignore
let fib = {
    struct Recurrence {
        mem: [u64; 2],
        pos: usize,
    }
```

This will be the actual iterator type.  `mem` will be the memo buffer to hold the last few values so the recurrence can be computed.  `pos` is to keep track of the value of `n`.

> **Aside**: I've chosen `u64` as a "sufficiently large" type for the elements of this sequence.  Don't worry about how this will work out for *other* sequences; we'll come to it.

```ignore
    impl Iterator for Recurrence {
        type Item = u64;

        #[inline]
        fn next(&mut self) -> Option<u64> {
            if self.pos < 2 {
                let next_val = self.mem[self.pos];
                self.pos += 1;
                Some(next_val)
```

We need a branch to yield the initial values of the sequence; nothing tricky.

```ignore
            } else {
                let a = /* something */;
                let n = self.pos;
                let next_val = (a[n-1] + a[n-2]);

                self.mem.TODO_shuffle_down_and_append(next_val);

                self.pos += 1;
                Some(next_val)
            }
        }
    }
```

This is a bit harder; we'll come back and look at *how* exactly to define `a`.  Also, `TODO_shuffle_down_and_append` is another placeholder; I want something that places `next_val` on the end of the array, shuffling the rest down by one space, dropping the 0th element.

```ignore

    Recurrence { mem: [1, 1], pos: 0 }
};

for e in fib.take(10) { println!("{}", e) }
```

Lastly, return an instance of our new structure, which can then be iterated over.  To summarise, the complete expansion is:

```ignore
let fib = {
    struct Recurrence {
        mem: [u64; 2],
        pos: usize,
    }

    impl Iterator for Recurrence {
        type Item = u64;

        #[inline]
        fn next(&mut self) -> Option<u64> {
            if self.pos < 2 {
                let next_val = self.mem[self.pos];
                self.pos += 1;
                Some(next_val)
            } else {
                let a = /* something */;
                let n = self.pos;
                let next_val = (a[n-1] + a[n-2]);

                self.mem.TODO_shuffle_down_and_append(next_val.clone());

                self.pos += 1;
                Some(next_val)
            }
        }
    }

    Recurrence { mem: [1, 1], pos: 0 }
};

for e in fib.take(10) { println!("{}", e) }
```

> **Aside**: Yes, this *does* mean we're defining a different `Recurrence` struct and its implementation for each macro invocation.  Most of this will optimise away in the final binary, with some judicious use of `#[inline]` attributes.

It's also useful to check your expansion as you're writing it.  If you see anything in the expansion that needs to vary with the invocation, but *isn't* in the actual macro syntax, you should work out where to introduce it.  In this case, we've added `u64`, but that's not neccesarily what the user wants, nor is it in the macro syntax.  So let's fix that.

```rust
macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ , ... , $recur:expr ) => { /* ... */ };
}

/*
let fib = recurrence![a[n]: u64 = 1, 1, ..., a[n-1] + a[n-2]];

for e in fib.take(10) { println!("{}", e) }
*/
# fn main() {}
```

Here, I've added a new capture: `sty` which should be a type.

> **Aside**: if you're wondering, the bit after the colon in a capture can be any of the following:
>
> - `item`: an item, like a function, struct, module, etc.
> - `block`: a block (*i.e.* a block of statments and/or an expression, surrounded by braces)
> - `stmt`: a statement
> - `pat`: a pattern
> - `expr`: an expression
> - `ty`: a type
> - `ident`: an identifier
> - `path`: a path (*e.g.* `foo`, `::std::mem::replace`, `transmute::<_, int>`, ...)
> - `meta`: a meta item; the things that go inside `#[...]` and `#![...]` attributes
> - `tt`: a single token tree
>
> Source: [`libsyntax/ext/tt/macro_parser.rs`](https://github.com/rust-lang/rust/blob/1.0.0/src/libsyntax/ext/tt/macro_parser.rs#L511-L557)
>
> There's one other thing to be aware of: in the interests of future-proofing the language, the compiler restricts what tokens you're allowed to put *after* a matcher, depending on what kind it is.  Typically, this comes up when trying to match expressions or statements; those can *only* be followed by one of `=>`, `,`, and `;`.
>
> Source [`libsyntax/ext/tt/macro_rules.rs`](https://github.com/rust-lang/rust/blob/1.0.0/src/libsyntax/ext/tt/macro_rules.rs#L430-L476)

# Indexing and Shuffling

I will skim a bit over this part, since it's effectively tangential to the macro stuff.  We want to make it so that the user can access previous values in the sequence by indexing `a`; we want it to act as a sliding window keeping the last few (in this case, 2) elements of the sequence.

We can do this pretty easily with a wrapper type:

```ignore
struct IndexOffset<'a> {
    slice: &'a [u64, ..2],
    offset: usize,
}

impl<'a> Index<usize> for IndexOffset<'a> {
    type Output = u64;

    #[inline(always)]
    fn index<'b>(&'b self, index: usize) -> &'b u64 {
        let real_index = *index - self.offset + 2;
        &self.slice[real_index]
    }
}
```

> **Aside**: since lifetimes come up *a lot* with people new to Rust, a quick explanation: `'a` and `'b` are lifetime parameters that are used to track where a reference (*i.e.* a borrowed pointer to some data) is valid.  In this case, `IndexOffset` borrows a reference to our iterator's data, so it needs to keep track of how long it's allowed to hold that reference for, using `'a`.
>
> `'b` is used because the `Index::index` function (which is how subscript syntax is actually implemented) is *also* parameterised on a lifetime, on account of returning a borrowed reference.  `'a` and `'b` are not necessarily the same thing in all cases.  The borrow checker will make sure that even though we don't explicitly relate `'a` and `'b` to one another, we don't accidentally violate memory safety.

This changes the definition of `a` to:

```ignore
let a = IndexOffset { slice: &self.mem, offset: n };
```

The only remaining question is what to do about `TODO_shuffle_down_and_append`.  I wasn't able to find a method in the standard library with exactly the semantics I wanted, but it isn't hard to do by hand.

```ignore
{
    use std::mem::swap;

    let mut swap_tmp = next_val;
    for i in (0..2).rev() {
        swap(&mut swap_tmp, &mut self.mem[i]);
    }
}
```

This swaps the new value into the end of the array, swapping the other elements down one space.

> **Aside**: doing it this way means that this code will work for non-copyable types, as well.

The working code thus far now looks like this:

```rust
macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ , ... , $recur:expr ) => { /* ... */ };
}

fn main() {
    /*
    let fib = recurrence![a[n]: u64 = 1, 1, ..., a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }
    */
    let fib = {
        use std::ops::Index;

        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }

        struct IndexOffset<'a> {
            slice: &'a [u64; 2],
            offset: usize,
        }

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;

            #[inline(always)]
            fn index<'b>(&'b self, index: usize) -> &'b u64 {
                use std::num::Wrapping;
                
                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(2);
                
                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }

        impl Iterator for Recurrence {
            type Item = u64;

            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let next_val = {
                        let n = self.pos;
                        let a = IndexOffset { slice: &self.mem, offset: n };
                        (a[n-1] + a[n-2])
                    };

                    {
                        use std::mem::swap;

                        let mut swap_tmp = next_val;
                        for i in (0..2).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }

                    self.pos += 1;
                    Some(next_val)
                }
            }
        }

        Recurrence { mem: [1, 1], pos: 0 }
    };

    for e in fib.take(10) { println!("{}", e) }
}
```

Note that I've changed the order of the declarations of `n` and `a`, as well as wrapped them (along with the recurrence expression) in a block.  The reason for the first should be obvious (`n` needs to be defined first so I can use it for `a`).  The reason for the second is that the borrowed reference `&self.mem` will prevent the swaps later on from happening (you cannot mutate something that is alised elsewhere).  The block ensures that the `&self.mem` borrow expires before then.

Incidentally, the only reason the code that does the `mem` swaps is in a block is to narrow the scope in which `std::mem::swap` is available, for the sake of being tidy.

If we take this code and run it, we get:

```text
1
1
2
3
5
8
13
21
34
55
```

Success!  Now, let's copy & paste this into the macro expansion, and replace the expanded code with an invocation.  This gives us:

```ignore
macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ , ... , $recur:expr ) => {
        {
            /*
                What follows here is *literally* the code from before,
                cut and pasted into a new position.  No other changes
                have been made.
            */

            use std::ops::Index;
    
            struct Recurrence {
                mem: [u64; 2],
                pos: usize,
            }
    
            struct IndexOffset<'a> {
                slice: &'a [u64; 2],
                offset: usize,
            }
    
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = u64;
    
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b u64 {
                    use std::num::Wrapping;
                    
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(2);
                    
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
    
            impl Iterator for Recurrence {
                type Item = u64;
    
                #[inline]
                fn next(&mut self) -> Option<u64> {
                    if self.pos < 2 {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset { slice: &self.mem, offset: n };
                            (a[n-1] + a[n-2])
                        };
    
                        {
                            use std::mem::swap;
    
                            let mut swap_tmp = next_val;
                            for i in (0..2).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
    
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
    
            Recurrence { mem: [1, 1], pos: 0 }
        }
    };
}

fn main() {
    let fib = recurrence![a[n]: u64 = 1, 1, ..., a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }
}
```

Obviously, we aren't *using* the captures yet, but we can change that fairly easily.  However, if we try to compile this, `rustc` aborts, telling us:

```text
recurrence.rs:69:45: 69:48 error: local ambiguity: multiple parsing options: built-in NTs expr ('inits') or 1 other options.
recurrence.rs:69     let fib = recurrence![a[n]: u64 = 1, 1, ..., a[n-1] + a[n-2]];
                                                             ^~~
```

Here, we've run into a limitation of `macro_rules`.  The problem is that second comma.  When it sees it during expansion, `macro_rules` can't decide if it's supposed to parse *another* expression for `inits`, or `...`.  Sadly, it isn't quite clever enough to realise that `...` isn't a valid expression, so it gives up.  Theoretically, this *should* work as desired, but currently doesn't.

> **Aside**: I *did* fib a little about how our rule would be interpreted by the macro system.  In general, it *should* work as described, but doesn't in this case.  The `macro_rules` machinery, as it stands, has its foibles, and its worthwhile remembering that on occasion, you'll need to contort a little to get it to work.
>
> On the bright side, this is a state of affairs that exactly *no one* is enthusiastic about.  The `macro` keyword has already been reserved for a more rigorously-defined future macro system.  Until then, needs must.

Thankfully, the fix is relatively simple: we remove the comma from the syntax.  To keep things balanced, we'll remove *both* commas around `...`:

```rust
macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
//                                     ^~~ changed
        /* ... */
#         // Cheat :D
#         (vec![1u64, 1, 2, 3, 5, 8, 13, 21, 34, 55]).into_iter()
    };
}

fn main() {
    let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
//                                         ^~~ changed

    for e in fib.take(10) { println!("{}", e) }
}
```

Success!  We can now start replacing things in the *expansion* with things we've *captured*.

## Substitution

Substituting something you've captured in a macro is quite simple; you can insert the contents of a capture `$sty:ty` by using `$sty`.  So, let's go through and fix the `u64`s:

```rust
macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
        {
            use std::ops::Index;
    
            struct Recurrence {
                mem: [$sty; 2],
//                    ^~~~ changed
                pos: usize,
            }
    
            struct IndexOffset<'a> {
                slice: &'a [$sty; 2],
//                          ^~~~ changed
                offset: usize,
            }
    
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;
//                            ^~~~ changed
    
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
//                                                          ^~~~ changed
                    use std::num::Wrapping;
                    
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(2);
                    
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
    
            impl Iterator for Recurrence {
                type Item = $sty;
//                          ^~~~ changed
    
                #[inline]
                fn next(&mut self) -> Option<$sty> {
//                                           ^~~~ changed
                    /* ... */
#                     if self.pos < 2 {
#                         let next_val = self.mem[self.pos];
#                         self.pos += 1;
#                         Some(next_val)
#                     } else {
#                         let next_val = {
#                             let n = self.pos;
#                             let a = IndexOffset { slice: &self.mem, offset: n };
#                             (a[n-1] + a[n-2])
#                         };
#     
#                         {
#                             use std::mem::swap;
#     
#                             let mut swap_tmp = next_val;
#                             for i in (0..2).rev() {
#                                 swap(&mut swap_tmp, &mut self.mem[i]);
#                             }
#                         }
#     
#                         self.pos += 1;
#                         Some(next_val)
#                     }
                }
            }
    
            Recurrence { mem: [1, 1], pos: 0 }
        }
    };
}

fn main() {
    let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }
}
```

Let's tackle a harder one: how to turn `inits` into both the array literal `[1, 1]` *and* the array type, `[$sty; 2]`.  The first one we can do like so:

```ignore
            Recurrence { mem: [$($inits),+], pos: 0 }
//                             ^~~~~~~~~~~ changed
```

This effectively does the opposite of the capture: repeat `inits` one or more times, separating each with a comma.  This expands to the expected sequence of tokens: `1, 1`.

Somehow turning `inits` into a literal `2` is a little trickier.  It turns out that there's no direct way to do this, but we *can* do it by using a second macro.  Let's take this one step at a time.

```rust
macro_rules! count_exprs {
    /* ??? */
#     () => {}
}
# fn main() {}
```

The obvious case is: given zero expressions, you would expect `count_exprs` to expand to a literal `0`.

```rust
macro_rules! count_exprs {
    () => (0);
//  ^~~~~~~~~~ added
}
# fn main() {
#     const _0: usize = count_exprs!();
#     assert_eq!(_0, 0);
# }
```

> **Aside**: You may have noticed I used parentheses here instead of curly braces for the expansion.  `macro_rules` really doesn't care *what* you use, so long as it's one of the "matcher" pairs: `( )`, `{ }` or `[ ]`.  In fact, you can switch out the matchers on the macro itself (*i.e.* the matchers right after the macro name), the matchers around the syntax rule, and the matchers around the corresponding expansion.
>
> You can also switch out the matchers used when you *invoke* a macro, but in a more limited fashion: a macro invoked as `{ ... }` or `( ... );` will *always* be parsed as an *item* (*i.e.* like a `struct` or `fn` declaration).  This is important when using macros in a function body; it helps disambiguate between "parse like an expression" and "parse like a statement".

What if you have *one* expression?  That should be a literal `1`.

```rust
macro_rules! count_exprs {
    () => (0);
    ($e:expr) => (1);
//  ^~~~~~~~~~~~~~~~~ added
}
# fn main() {
#     const _0: usize = count_exprs!();
#     const _1: usize = count_exprs!(x);
#     assert_eq!(_0, 0);
#     assert_eq!(_1, 1);
# }
```

Two?

```rust
macro_rules! count_exprs {
    () => (0);
    ($e:expr) => (1);
    ($e0:expr, $e1:expr) => (2);
//  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~ added
}
# fn main() {
#     const _0: usize = count_exprs!();
#     const _1: usize = count_exprs!(x);
#     const _2: usize = count_exprs!(x, y);
#     assert_eq!(_0, 0);
#     assert_eq!(_1, 1);
#     assert_eq!(_2, 2);
# }
```

We can "simplify" this a little by re-expressing the case of two expressions recursively.

```rust
macro_rules! count_exprs {
    () => (0);
    ($e:expr) => (1);
    ($e0:expr, $e1:expr) => (1 + count_exprs!($e1));
//                           ^~~~~~~~~~~~~~~~~~~~~ changed
}
# fn main() {
#     const _0: usize = count_exprs!();
#     const _1: usize = count_exprs!(x);
#     const _2: usize = count_exprs!(x, y);
#     assert_eq!(_0, 0);
#     assert_eq!(_1, 1);
#     assert_eq!(_2, 2);
# }
```

This is fine since Rust can fold `1 + 1` into a constant value.  What if we have three expressions?

```rust
macro_rules! count_exprs {
    () => (0);
    ($e:expr) => (1);
    ($e0:expr, $e1:expr) => (1 + count_exprs!($e1));
    ($e0:expr, $e1:expr, $e2:expr) => (1 + count_exprs!($e1, $e2));
//  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ added
}
# fn main() {
#     const _0: usize = count_exprs!();
#     const _1: usize = count_exprs!(x);
#     const _2: usize = count_exprs!(x, y);
#     const _3: usize = count_exprs!(x, y, z);
#     assert_eq!(_0, 0);
#     assert_eq!(_1, 1);
#     assert_eq!(_2, 2);
#     assert_eq!(_3, 3);
# }
```

> **Aside**: You might be wondering if we could reverse the order of these rules.  In this particular case, *yes*, but the macro system can sometimes be picky about what it is and is not willing to recover from.  If you ever find yourself with a multi-rule macro that you *swear* should work, but gives you errors about unexpected tokens, try changing the order of the rules.

Hopefully, you can see the pattern here.  We can always reduce the list of expressions by matching one expression, followed by zero or more expressions, expanding that into 1 + a count.

```rust
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
//  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ changed
}
# fn main() {
#     const _0: usize = count_exprs!();
#     const _1: usize = count_exprs!(x);
#     const _2: usize = count_exprs!(x, y);
#     const _3: usize = count_exprs!(x, y, z);
#     assert_eq!(_0, 0);
#     assert_eq!(_1, 1);
#     assert_eq!(_2, 2);
#     assert_eq!(_3, 3);
# }
```

With this, we can now modify `recurrence` to determine the necessary size of `mem`.

> **Note**: The lexical ordering of `count_exprs` and `recurrence` is important.  This will be expounded upon at the end.

```rust
// added:
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
        {
            use std::ops::Index;
            
            const MEM_SIZE: usize = count_exprs!($($inits),+);
//          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ added
    
            struct Recurrence {
                mem: [$sty; MEM_SIZE],
//                          ^~~~~~~~ changed
                pos: usize,
            }
    
            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
//                                ^~~~~~~~ changed
                offset: usize,
            }
    
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;
    
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;
                    
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);
//                                        ^~~~~~~~ changed
                    
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
    
            impl Iterator for Recurrence {
                type Item = $sty;
    
                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
//                                ^~~~~~~~ changed
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset { slice: &self.mem, offset: n };
                            (a[n-1] + a[n-2])
                        };
    
                        {
                            use std::mem::swap;
    
                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
//                                       ^~~~~~~~ changed
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
    
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
    
            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}
/* ... */
# 
# fn main() {
#     let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
# 
#     for e in fib.take(10) { println!("{}", e) }
# }
```

With that done, we can now substitute the last thing: the `recur` expression.

```ignore
# macro_rules! count_exprs {
#     () => (0);
#     ($head:expr $(, $tail:expr)*) => (1 + count_exprs!($($tail),*));
# }
# macro_rules! recurrence {
#     ( a[n]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
#         {
#             const MEMORY: uint = count_exprs!($($inits),+);
#             struct Recurrence {
#                 mem: [$sty, ..MEMORY],
#                 pos: uint,
#             }
#             struct IndexOffset<'a> {
#                 slice: &'a [$sty, ..MEMORY],
#                 offset: uint,
#             }
#             impl<'a> Index<uint, $sty> for IndexOffset<'a> {
#                 #[inline(always)]
#                 fn index<'b>(&'b self, index: &uint) -> &'b $sty {
#                     let real_index = *index - self.offset + MEMORY;
#                     &self.slice[real_index]
#                 }
#             }
#             impl Iterator<u64> for Recurrence {
/* ... */
                #[inline]
                fn next(&mut self) -> Option<u64> {
                    if self.pos < MEMORY {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset { slice: &self.mem, offset: n };
                            $recur
//                          ^~~~~~ changed
                        };
                        {
                            use std::mem::swap;
                            let mut swap_tmp = next_val;
                            for i in range(0, MEMORY).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
                        self.pos += 1;
                        Some(next_val)
                    }
                }
/* ... */
#             }
#             Recurrence { mem: [$($inits),+], pos: 0 }
#         }
#     };
# }
# fn main() {
#     let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
#     for e in fib.take(10) { println!("{}", e) }
# }
```

And, when we compile our finished macro...

```text
recurrence.rs:77:48: 77:49 error: unresolved name `a`
recurrence.rs:77     let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
                                                                ^
recurrence.rs:7:1: 74:2 note: in expansion of recurrence!
recurrence.rs:77:15: 77:64 note: expansion site
recurrence.rs:77:50: 77:51 error: unresolved name `n`
recurrence.rs:77     let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
                                                                  ^
recurrence.rs:7:1: 74:2 note: in expansion of recurrence!
recurrence.rs:77:15: 77:64 note: expansion site
recurrence.rs:77:57: 77:58 error: unresolved name `a`
recurrence.rs:77     let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
                                                                         ^
recurrence.rs:7:1: 74:2 note: in expansion of recurrence!
recurrence.rs:77:15: 77:64 note: expansion site
recurrence.rs:77:59: 77:60 error: unresolved name `n`
recurrence.rs:77     let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];
                                                                           ^
recurrence.rs:7:1: 74:2 note: in expansion of recurrence!
recurrence.rs:77:15: 77:64 note: expansion site
```

... wait, what?  That can't be right... let's check what the macro is expanding to.

```shell
$ rustc -Z unstable-options --pretty expanded recurrence.rs
```

The `--pretty expanded` argument tells `rustc` to perform macro expansion, then turn the resulting AST back into source code.  Because this option isn't considered stable yet, we also need `-Z unstable-options`.  The output (after cleaning up some formatting) is shown below; in particular, note the place in the code where `$recur` was substituted:

```ignore
#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
fn main() {
    let fib = {
        use std::ops::Index;
        const MEM_SIZE: usize = 1 + 1;
        struct Recurrence {
            mem: [u64; MEM_SIZE],
            pos: usize,
        }
        struct IndexOffset<'a> {
            slice: &'a [u64; MEM_SIZE],
            offset: usize,
        }
        impl <'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;
            #[inline(always)]
            fn index<'b>(&'b self, index: usize) -> &'b u64 {
                use std::num::Wrapping;
                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(MEM_SIZE);
                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }
        impl Iterator for Recurrence {
            type Item = u64;
            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.pos < MEM_SIZE {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let next_val = {
                        let n = self.pos;
                        let a = IndexOffset{slice: &self.mem, offset: n,};
                        a[n - 1] + a[n - 2]
                    };
                    {
                        use std::mem::swap;
                        let mut swap_tmp = next_val;
                        {
                            let result =
                                match ::std::iter::IntoIterator::into_iter((0..MEM_SIZE).rev()) {
                                    mut iter => loop {
                                        match ::std::iter::Iterator::next(&mut iter) {
                                            ::std::option::Option::Some(i) => {
                                                swap(&mut swap_tmp, &mut self.mem[i]);
                                            }
                                            ::std::option::Option::None => break,
                                        }
                                    },
                                };
                            result
                        }
                    }
                    self.pos += 1;
                    Some(next_val)
                }
            }
        }
        Recurrence{mem: [1, 1], pos: 0,}
    };
    {
        let result =
            match ::std::iter::IntoIterator::into_iter(fib.take(10)) {
                mut iter => loop {
                    match ::std::iter::Iterator::next(&mut iter) {
                        ::std::option::Option::Some(e) => {
                            ::std::io::_print(::std::fmt::Arguments::new_v1(
                                {
                                    static __STATIC_FMTSTR: &'static [&'static str] = &["", "\n"];
                                    __STATIC_FMTSTR
                                },
                                &match (&e,) {
                                    (__arg0,) => [::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Display::fmt)],
                                }
                            ))
                        }
                        ::std::option::Option::None => break,
                    }
                },
            };
        result
    }
}
```

But that looks fine!  If we add a few missing `#![feature(...)]` attributes and feed it to a nightly build of `rustc`, it even compiles!  ... *what?!*

> **Aside**: You can't compile the above with a non-nightly build of `rustc`.  This is because the expansion of the `println!` macro depends on internal compiler details which are *not* publically stabilised.

## Being Hygienic

The issue here is that identifiers in Rust macros are *hygienic*.  That is, identifiers from two different contexts *cannot* collide.  To show the difference, let's take a simpler example.

```rust
# /*
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42i;
            $e
        }
    }
}

let four = using_a!(a / 10);
# */
# fn main() {}
```

This macro simply takes an expression, then wraps it in a block with a variable `a` defined.  We then use this as a round-about way of computing `4`.  There are actually *two* syntax contexts in this example, but they're invisible.  So, to help with this, let's give each context a different colour:

<style type="text/css">.sc-0 { background-color: #aaf; }</style>
<style type="text/css">.sc-1 { background-color: #faa; }</style>

<pre class="rust"><span class="sc-0">macro_rules! using_a {
    ($e:expr) => {</span><span class="sc-1">
        {
            let a = 42i;
            $e
        }
    </span><span class="sc-0">}
}

let four = using_a!(a / 10);</span>
</pre>

Now, let's expand the invocation.

<pre class="rust"><span class="sc-0">macro_rules! using_a {
    ($e:expr) => {</span><span class="sc-1">
        {
            let a = 42i;
            $e
        }
    </span><span class="sc-0">}
}

let four = </span><span class="sc-1">{
    let a = 42i;
    </span><span class="sc-0">a / 10</span><span class="sc-1">
}</span><span class="sc-0">;</span>
</pre>

As you can see, the <code class="sc-1">a</code> that's defined by the macro is in a different context to the <code class="sc-0">a</code> we provided in our invocation.  As such, the compiler treats them as completely different identifiers, *even though they have the same lexical appearance*.

This is something to be *really* careful of when working on macros: macros can produce ASTs which will not compile, but which *will* compile if written out by hand, or dumped using `--pretty expanded`.

The solution to this is to capture the identifier *with the appropriate syntax context*.  To do that, we need to again adjust our macro syntax.  To continue with our simpler example:

<pre class="rust"><span class="sc-0">macro_rules! using_a {</span><span class="sc-1">
    ($a:ident, $e:expr) => {
        {
            let $a = 42i;
            $e
        }
    }
</span><span class="sc-0">}

let four = using_a!(a, a / 10);</span>
</pre>

This now expands to:

<pre class="rust"><span class="sc-0">macro_rules! using_a {</span><span class="sc-1">
    ($a:ident, $e:expr) => {
        {
            let $a = 42i;
            $e
        }
    }
</span><span class="sc-0">}

let four = </span><span class="sc-1">{
    let </span><span class="sc-0">a</span><span class="sc-1"> = 42i;
    </span><span class="sc-0">a / 10</span><span class="sc-1">
}</span><span class="sc-0">;</span>
</pre>

Now, the contexts match, and the code will compile.  We can make this adjustment to our `recurrence!` macro by explicitly capturing `a` and `n`.  After making the necessary changes, we have:

```rust
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
//    ^~~~~~~~~~   ^~~~~~~~~~ changed
        {
            use std::ops::Index;
            
            const MEM_SIZE: usize = count_exprs!($($inits),+);
    
            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }
    
            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }
    
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;
    
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;
                    
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);
                    
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
    
            impl Iterator for Recurrence {
                type Item = $sty;
    
                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
//                              ^~~~ changed
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
//                              ^~~~ changed
                            $recur
                        };
    
                        {
                            use std::mem::swap;
    
                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
    
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
    
            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

fn main() {
    let fib = recurrence![a[n]: u64 = 1, 1 ... a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }
}
```

And it compiles!  Now, let's try with a different sequence.

```rust
# macro_rules! count_exprs {
#     () => (0);
#     ($head:expr) => (1);
#     ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
# }
# 
# macro_rules! recurrence {
#     ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
#         {
#             use std::ops::Index;
#             
#             const MEM_SIZE: usize = count_exprs!($($inits),+);
#     
#             struct Recurrence {
#                 mem: [$sty; MEM_SIZE],
#                 pos: usize,
#             }
#     
#             struct IndexOffset<'a> {
#                 slice: &'a [$sty; MEM_SIZE],
#                 offset: usize,
#             }
#     
#             impl<'a> Index<usize> for IndexOffset<'a> {
#                 type Output = $sty;
#     
#                 #[inline(always)]
#                 fn index<'b>(&'b self, index: usize) -> &'b $sty {
#                     use std::num::Wrapping;
#                     
#                     let index = Wrapping(index);
#                     let offset = Wrapping(self.offset);
#                     let window = Wrapping(MEM_SIZE);
#                     
#                     let real_index = index - offset + window;
#                     &self.slice[real_index.0]
#                 }
#             }
#     
#             impl Iterator for Recurrence {
#                 type Item = $sty;
#     
#                 #[inline]
#                 fn next(&mut self) -> Option<$sty> {
#                     if self.pos < MEM_SIZE {
#                         let next_val = self.mem[self.pos];
#                         self.pos += 1;
#                         Some(next_val)
#                     } else {
#                         let next_val = {
#                             let $ind = self.pos;
#                             let $seq = IndexOffset { slice: &self.mem, offset: $ind };
#                             $recur
#                         };
#     
#                         {
#                             use std::mem::swap;
#     
#                             let mut swap_tmp = next_val;
#                             for i in (0..MEM_SIZE).rev() {
#                                 swap(&mut swap_tmp, &mut self.mem[i]);
#                             }
#                         }
#     
#                         self.pos += 1;
#                         Some(next_val)
#                     }
#                 }
#             }
#     
#             Recurrence { mem: [$($inits),+], pos: 0 }
#         }
#     };
# }
# 
# fn main() {
for e in recurrence!(f[i]: f64 = 1.0 ... f[i-1] * i as f64).take(10) {
    println!("{}", e)
}
# }
```

Which gives us:

```text
1
1
2
6
24
120
720
5040
40320
362880
```

Success!

# The Nuclear Option

There *is* another way to create a macro for Rust: write a "syntax extension".  This is a regular Rust library that links aginst `libsyntax` and is loaded by the compiler at compile time.  Going further into this is completely beyond the scope of this article.  If you want to learn more, you can read the [Compiler Plugins chapter of the Rust Book](http://doc.rust-lang.org/stable/book/compiler-plugins.html), but even then be prepared to do a lot of code spelunking!

That said, syntax extensions can do pretty much anything, as they have full access to the AST, the compiler's parser, and all the usual Rust libraries.  As a *very* minimal example, a syntax extension could have worked around that *trifling* business with the commas and `...`.  If you enjoy metaprogramming, it may be worthwhile to take a stab at them.

They *do* come with one rather significant drawback, however.  *Because* they need to link against `libsyntax`, and the interface of `libsyntax` is inherently an internal compiler detail, syntax extensions *cannot* be used in stable or beta versions of the compiler: they will *only* work in nightly builds.  This is to prevent people from accidentally depending on unstable details in stable code.

# Some More Gotchas

Before we move on, it's worth covering some important differences between macros and other items in Rust.

**Macros are order-dependent.** That means that you *cannot* use a macro *before* it has been defined.

**Macros are (sometimes) lexically-scoped.** This is a bit weird in Rust.  Remember that all Rust crates are really just a single, giant file often *pretending* to be multiple files.  This means that the following will happen:

<table>
    <tr>
        <td><code>lib.rs</code></td>
        <td><pre class="rust">/* X is *not* defined. */
mod a;
/* X is *not* defined. */
mod b;
/* X is *not* defined. */
mod c;
/* X is *not* defined. */</pre></td>
    </tr>
    <tr>
        <td><code>a.rs</code></td>
        <td><pre class="rust">/* X is *not* defined. */</pre></td>
    </tr>
    <tr>
        <td><code>b.rs</code></td>
        <td><pre class="rust">/* ... */
/* X is *not* defined. */
macro_rules! X { () => ("X") }
/* X *is* defined. */
/* ... */</pre></td>
    </tr>
    <tr>
        <td><code>c.rs</code></td>
        <td><pre class="rust">/* X is *not* defined. */</pre></td>
    </tr>
</table>

However, we can alter this behaviour by applying the `#[macro_use]` attribute like so:

<table>
    <tr>
        <td><code>lib.rs</code></td>
        <td><pre class="rust">/* X is *not* defined. */
mod a;
/* X is *not* defined. */
#[macro_use] mod b;
/* X *is* defined. */
mod c;
/* X *is* defined. */</pre></td>
    </tr>
    <tr>
        <td><code>a.rs</code></td>
        <td><pre class="rust">/* X is *not* defined. */</pre></td>
    </tr>
    <tr>
        <td><code>b.rs</code></td>
        <td><pre class="rust">/* ... */
/* X is *not* defined. */
macro_rules! X { () => ("X") }
/* X *is* defined. */
/* ... */</pre></td>
    </tr>
    <tr>
        <td><code>c.rs</code></td>
        <td><pre class="rust">/* X *is* defined. */</pre></td>
    </tr>
</table>

Note that the definition of `X` cuts clear through the module heirarchy and across files.  For this reason, I personally recommend that you always define macros in the *first* module defined in a library, or at the top of your `lib.rs` or `main.rs` file.

**Macros are individually exported.** In order to make a macro available to other crates, you have to use the `#[macro_export]` attribute.  Note that this and `#[macro_use]` are *unrelated*.  You can have a macro exported to other crates, but which is not available to most of your own crate.  So, for example:

```rust
#[macro_export]
macro_rules! X { () => ("X") }
# fn main() {}
```

This would make `X` available to other crates.  However...

**Macros aren't regular items.** That is, you cannot access macros simply by using `extern crate stuff;`.  Instead, you need to use `#[macro_use] extern crate stuff;`.  This tells the compiler that it needs to load `libstuff` at compile time, in addition to at runtime.

You can also limit which macros you pull in from an external crate by specifying their names like so: `#[macro_use(macro_a, thingy="macro_b")] extern crate stuff;`.

# Distribution

So, we have a nice little `recurrence` macro, and we'd like to make it available for other people to use.  How do we do this?

The best approach is to package it up in a Cargo package and put it online somewhere.  Drop into a terminal and create a new Cargo package like so:

```shell
$ cargo new recurrence
```

This should create a `recurrence/.gitignore`, `recurrence/Cargo.toml` and `recurrence/src/lib.rs`.  You will need to modify the `Cargo.toml` to look like this:

```toml
[package]

name = "recurrence"
version = "0.1.0"
authors = ["Writer McAuthor <writer.mcauthor@mail.com>"]

[lib]

name = "recurrence"
crate-type = ["dylib"]
plugin = true
```

The really important part is `plugin = true`.  This tells Cargo that it needs to make the compiled crate available to the compiler.  Next, we'll fill out `src/lib.rs`:

```ignore
//! This crate defines a macro for creating iterators which implement
//! recurrence relations.

#[doc(hidden)]
#[macro_export]
macro_rules! _recurrence_count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + _recurrence_count_exprs!($($tail),*));
}

/// Expands to an expression implementing the `Iterator` trait, which yields
/// successive elements of the given recurrence relationship.
///
/// For example, you can define a Fibonacci sequence iterator like so:
/// 
/// ```rust
/// # #[macro_use] extern crate recurrence;
/// # fn main() {
/// #     let _ =
/// recurrence![ fib[n]: f64 = 0.0, 1.0 ... fib[n-1] + fib[n-2] ]
/// #     ;
/// # }
/// ```
#[macro_export]
macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
        {
            use std::ops::Index;
            
            const MEM_SIZE: usize = _recurrence_count_exprs!($($inits),+);
    
            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }
    
            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }
    
            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;
    
                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;
                    
                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);
                    
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }
    
            impl Iterator for Recurrence {
                type Item = $sty;
    
                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur
                        };
    
                        {
                            use std::mem::swap;
    
                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
    
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }
    
            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}
# fn main() {}
```

Note that `count_exprs!` actually causes us a small problem.  Because it's in the *expansion* of `recurrence!`, it has to be available from the invocation site.  This means we *have* to export it.  However, it's really just an implementation detail.  As such, I've given it a more unique name, exported it, and hidden it from the docs.  We've also exported `recurrence!` itself since there's obviously not much point if we *don't*.

I've also added a quick example.  If you're wondering why there are all those `#`s, it's because example code blocks are, by default, also considered to be executable tests.  The `#`s hide lines that are needed for the test to execute, but which aren't relevant to the actual, rendered documentation.

Now that said, Cargo does not currently run doc-comment tests, so we'll duplicate it in an actual integration test; create `tests/fib-10.rs`, and put the following in it:

```ignore
#[macro_use] extern crate recurrence;

#[test]
fn test_fib_10() {
    let fib = recurrence![ fib[n]: f64 = 0.0, 1.0 ... fib[n-1] + fib[n-2] ];
    let fib_10: Vec<_> = fib.take(10).collect();
    assert_eq!(&*fib_10, &[0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0]);
}
```

With that done, you can use `cargo test` to make sure the crate compiles and passes its one and only test, and `cargo doc` to generate the documentation and see if it looks alright.

All you need to do now is to commit the changes to the repository, and publish it somewhere publically accessible (or privately, if you aren't interesting in letting other people use your awesome macro).  To use the crate, you just need to add a dependency to it in your crate's `Cargo.toml` and then link to it.  For example, [the completed recurrence crate](https://github.com/DanielKeep/rust-recurrence) is published as a Git repository.  To use it, just add it as a Cargo dependency.

Now, go forth and metaprogram!

# Postscript

Thanks to `snake_case`, `Yurume`, `Rym`, and `ogham` for providing feedback.

This work is licensed under both the [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/) and the [MIT license](http://opensource.org/licenses/MIT).
