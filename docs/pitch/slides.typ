#import "@preview/polylux:0.4.0": *
#import "@preview/fletcher:0.5.8": *
// #import themes.simple: *;

// #show: simple-theme.with(
//   aspect-ratio: "16-9",
//   primary: color.rgb("#D34516"),
// )

#set text(
  lang: "en",
  region: "us",
  font: "Inter",
  size: 60pt
)

#show raw: set text(
  size: 40pt,
)

#set page(
    paper: "presentation-16-9",
    width: 67.74cm,
    height: 38.1cm
)


#slide[
  #place(bottom, dy: -1.5em)[
    #text("Group 7", size: 60pt)
    #heading(text("crabdrive", fill: color.rgb("#D34516")))
    #v(.2em)
    A cloud storage platform
  ]
]

#slide[
  #underline([Core features])

  - Web interface for viewing and managing files
  - Sharing files with other users
  - File versioning

  #underline([Optional features])

  - End-To-End Encryption
  - Collaborative Editing
]


#slide[
  #underline([Architecture (Server-Request Handling)])


  #figure[
    #diagram(
      node-stroke: 3pt,
      node-inset: 20pt,
      node-corner-radius: 4mm,
      edge-stroke: 4pt,
      spacing: (3em, .5em),
      {
        node((0, 0), [Middleware], name: <middleware>, width: 140mm)
        node((.65, 0), stroke: none, align(left, text(
          [Authentication, \ Rate Limiting],
          size: 30pt,
          fill: gray.darken(30%),
        )))
        node((0, 3), [File Manager], name: <manager>, width: 150mm)
        node(
          enclose: ((1, 2.25), (1, 3.75)),
          place(top + left, text("Cache Layer", size: 45pt), dy: -3cm, dx: -1cm),
          stroke: (dash: "dashed"),
          inset: .5em
        )
        node((1, 2.25), [File], name: <filecache>, width: 120mm)
        node((1, 3.75), [Metadata], name: <metadatacache>, width: 120mm)
        node(
          (2, 4.5),
          [
            #v(.4em)
            #align(center, text("Database", size: .9em))
          ],
          width: 150mm,
          height: 40mm,
          shape: shapes.cylinder.with(rings: 10%),
          name: <database>,
        )
        node(
          (2, 1.5),
          [
            #v(.4em)
            #align(center, text("File System", size: .9em))
          ],
          width: 150mm,
          height: 40mm,
          shape: shapes.cylinder.with(rings: (7%, 14%)),
          name: <disk>,
        )

        edge(<middleware.south>, "-|>", <manager.north>, label: "")
        edge(<manager.east>, <filecache.west>, "-|>")
        edge(<manager.east>, "-|>", <metadatacache.west>)

        edge(<filecache.east>, "-|>", <disk.west>)
        edge(<metadatacache.east>, "-|>", <database.west>)

        edge((0, 0), (0, -3), "-", dash: "dotted")
      },
    )
  ]

]

// #show: appendix

#slide[

  #place(center + horizon)[
    #rotate(-45deg)[
      #text("CLASSIFIED", fill: gray.lighten(70%), size: 200pt, font: "Inter Display", tracking: 6mm)
    ]
  ]

  #place(top + right)[
    #text([*DO NOT SHARE*], size: 30pt, weight: "medium", fill: color.rgb("#D34516"), tracking: .35em)
  ]


  *Notes*
  #set text(size: 40pt)

  - crabdrive should behave similarly to other Cloud Platform providers (Google Drive, OneDrive, etc.)
  - We use *`axum`* or *`arctix`* as a server
  - The filetree is stored inside a SQL Database
]
