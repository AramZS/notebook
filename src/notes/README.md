---
aliases: ['Note Publisher', 'NotePub', 'NotePublisher', 'Notebook']
public: true
---

# Notebook

tags:: #fellowship-of-the-link #six-tools 
up:: [[FellowhipOfTheLink|Fellowship of the Link]] [[shared-notes-across-six-tools|Shared Notes Across Six Tools]]

A tool to publish those notes here that are intended to be public facing. 

## Directives for Parsing and Build: 

### YAML

- `define:` Show the Wikipedia definition 
- `public:` This note is intended to be published. 
	- Notes marked `false` or with no value will never have any part published. 
	- Notes marked `true` will have their whole body published along with metadata. 
	- Notes marked `partial-public` will have any `:::{public} content :::` content blocks published and no metadata other than title. 
	- Notes marked `partial-private` will have all content except for `:::{private} content :::` content blocks published and all metadata. 
	- Notes marked `true` will also follow the `private` directives. 
- `mainlink:` This note is referring to a URL as its primary subject. 
- `go:` This link defines the page, it is intended to be a forwarding link. 
- `isBasedOn:` This page is a direct other page and should be considered a replica or summary of that page. 
- `date:` intended publishing time. 
- `aliases:` YAML list of other ways to refer to this page besides filename. 
- `republish:` Designate a YAML list of TLDs or source list names which you wish to republish to. Processing of a republish event is up to the receiving site and may be an entirely new page or the absorption of the page into an existing page, or a denial. 
- `tags:`  to add keywords in a YAML list.  
- `moved-to:` Indicates this note file has been moved to a public-facing URL and that is the resource that should be accessed instead. 
- `mobed-date:` Indicates the date at which this file was moved to the public-facing URL. It never occurs without `moved-to`. 

### Folder Structure 

- A folder indicates hierarchy that indicates the contained pages should be considered a child of the Folder-as-a-page and that the Folder-as-a-page can list the `down` pages. 
- Any folder may have a relative `images`, `Images`, or `img` folder. 

### Metastatements 

- All meta statements are a single word ending in `:: `
- `tags::` add to `keywords`
- `up::` hierarchy that indicates this page should be considered a child of the listed pages and that the `up` pages can list the `down` pages. 
- `down::` hierarchy that indicates this page should be considered a parent of the listed pages. 
- `include::` Comma separated URL link of passed resources to be embedded at the end of the current link. 
- `circle::` Peer pages that are associated but not hierarchical. Order is irrelevant, but alphabetical is assumed.  

## Conventions 

- Wikilinks 
	- Should resolve to: aliases, titles, filenames minus `.md`, filenames minus folder structure if none remain. 
	- If no target page, create a page with the wikipedia definition 
- `#go url` This link are part of how we define this page
	- `#go/node/header/hashtag-or-node` will construct URLs based on header of the target node or the node itself. 
	- Read up: 
		- [On go links | Flancia](https://flancia.org/mine/go-links/)
			- [agora-server/db.py at main · flancian/agora-server · GitHub](https://github.com/flancian/agora-server/blob/main/app/db.py#L658)
			- [agora-server/db.py at main · flancian/agora-server · GitHub](https://github.com/flancian/agora-server/blob/main/app/db.py#L266)
		- [GoLinks® | Intuitive link management platform for teams](https://www.golinks.io/)
		- [Trotto - Open-Source Go Links](https://www.trot.to/)
			- [GitHub - trotto/browser-extension: A browser extension that makes the "go/" hostname work for any go links implementation. Built for easy self-hosting.](https://github.com/trotto/browser-extension)
		- `/f/` links on previous sites. 
		- Volunteer `go` links to the community. 
- `#pull url` This is a form of transclusion - it will result in pulling the passed resources or nodes into the resource in an embedded way (iframe maybe? archive?). This is an include-style statement. 
- `#calendar url` A calendar link for an event. 
- `#date datetime` An event datetime. 
- `#push nodename` Will push the `push` directive and anything listed underneath it in `li` elements to `nodename`.
- `:::{blocktype} content :::` designates special content blocks where the content is ` content ` in that example. The block can be multiline or single line. See: [MyST Markdown overview](https://datascientistforai.github.io/DataScienceStudy/content/myst.html#content-myst-directives)
- `|/` Indicates a Decision. 
- `- [ ]` Indicates a ToDo.
  - `#todo` before a block of `- [ ]` indicates globally visible ToDos. 
- Any item in the `/f/` folder at the base of the notes source is intended to be a forwarding link and should have a `go` directive in its YAML. 

## Interlinked

- Includes Context.center links as sources, adds them as searchable.
- Creates pages from Linked terms with a list of links that match that term. 
	- If this page is *only* a single link, consider that link to be the same as `isBasedOn`
- Links should include checking for `-` and `_` versions of the link.
- Pull in archive somehow? `#pull` creates an archive? 
- Pull in any public Google Doc as text for the page? 
- Establish list of sources 
	- Where those sources have url/matched-path - pull that source in as a link and potentially a node for the matched path on my own site. 
	- Can also establish a source name and use it as a path/selector: `[@rel8/foo]` where `@` points to a listing in the `sources` list. 
	- Agora source links: [agora/sources.yaml at master · flancian/agora · GitHub](https://github.com/flancian/agora/blob/master/sources.yaml)
	- Establish a URL resolution alias file for internal/external use? 

## Outputs 

- RSS feed of new pages 
- JSON API for page list? 
- JSON statement for url paths `index.html` and `index.json` 
- Raw MD version for url paths `index.html` and `index.md` 

## Useful Specs 

- [https://anagora.org/web-annotation-standard](https://anagora.org/web-annotation-standard) 
- [https://www.w3.org/TR/annotation-model/](https://www.w3.org/TR/annotation-model/)
