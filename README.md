# [Rust Maven in Hebrew](https://rust-he.code-maven.com/)


This repostory contains the source of all the articles and all the examples of the [Rust Maven in Hebrew](https://rust-he.code-maven.com/) web site.

Feel free to use any of the examples under the MIT OR Apache-2.0 license.

## Generate the site

Download `code-maven` from https://ssg.code-maven.com/

Run

```
code-maven web
```

It will generate the site in the `_site` folder.

## View site locally

Install [rustatic](https://rustatic.code-maven.com/)

and after generating the static pages with the previously described command, run

```
rustatic --host localhost --port 5000 --indexfile index.html --nice --path _site/
```
