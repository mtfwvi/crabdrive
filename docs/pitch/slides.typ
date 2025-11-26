#import "@preview/touying:0.6.1" : *
#import "@preview/fletcher:0.5.8" : *
#import themes.simple : *;

#show: simple-theme.with(aspect-ratio: "16-9")

#set text(
  lang: "en",
  region: "us",
  font: "Inter"
)


#slide[
  #place(bottom, dy: -1.5em)[
    #text("Group 7", size: 25pt)
    = crabdrive
    #v(.2em)
    A cloud storage platform
  ]
]

#slide[
  #underline([*Core features*])

  - Storing & retrieving files using a web browser
  - Presenting files in a explorer-like interface
  - File versioning

  #underline([*Optional features*])

  - E2E -- Encryption
  - Collaborative Editing
  - Search in file contents
]

