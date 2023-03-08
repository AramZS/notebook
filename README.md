---
aliases: ['Note Publisher', 'NotePub', 'NotePublisher']
public: true
---

# Notebook - A site for note publishing

Repo where my live public-facing notebook lives.

A tool to publish those notes here that are intended to be public facing. 

## Directives for Parsing and Build: 

### YAML

- `define:` Show the Wikipedia definition 
- `public:` This note is intended to be published. No note will be published without it 
- `mainlink:` This note is referring to a URL as its primary subject. 
- `go:` This link are part of how we define this page
- `isBasedOn:` This page is a direct other page and should be considered a replica or summary of that page. 
- `date:` intended publishing time. 

### Folder Structure 

- A folder indicates hierarchy that indicates the contained pages should be considered a child of the Folder-as-a-page and that the Folder-as-a-page can list the `down` pages. 
- Any folder may have a relative `images`, `Images`, or `img` folder. 

### Metastatements 

- All meta statements are a single word ending in `:: `
- `tags::` add to `keywords`
- `up::` hierarchy that indicates this page should be considered a child of the listed pages and that the `up` pages can list the `down` pages. 
- `down::` hierarchy that indicates this page should be considered a parent of the listed pages. 
- `include::` Comma separated URL link of passed resources to be embedded at the end of the current link. 

## Conventions 

- Wikilinks 
	- Should resolve to: aliases, titles, filenames minus `.md`, filenames minus folder structure if none remain. 
	- If no target page, create a page with the wikipedia definition 
- `#go url` This link are part of how we define this page
- `#pull url` This is a form of transclusion - it will result in pulling the passed resources or nodes into the resource in an embedded way (iframe maybe? archive?). This is an include-style statement. 
- `#calendar url` A calendar link for an event. 
- `#date datetime` An event datetime. 
- `#push nodename` Will push the `push` directive and anything listed underneath it in `li` elements to `nodename`.

## Interlinked

- Includes Context.center links as sources, adds them as searchable.
- Creates pages from Linked terms with a list of links that match that term. 
	- If this page is *only* a single link, consider that link to be the same as `isBasedOn`
- Pull in archive somehow? `#pull` creates an archive? 
- Pull in any public Google Doc as text for the page? 

## Outputs 

- RSS feed of new pages 
- JSON API for page list? 
- JSON statement for url paths `index.html` and `index.json` 
- Raw MD version for url paths `index.html` and `index.md` 

## Useful Specs 

- [https://anagora.org/web-annotation-standard](https://anagora.org/web-annotation-standard) 
- [https://www.w3.org/TR/annotation-model/](https://www.w3.org/TR/annotation-model/)
