#import "@preview/touying:0.6.1": *
#import "@preview/fletcher:0.5.8": *
#import themes.simple: *;

#show: simple-theme.with(
  aspect-ratio: "16-9",
  primary: color.rgb("#D34516"),
)

#set text(
  lang: "en",
  region: "us",
  font: "Inter",
)

#show raw: set text(
  size: 16pt,
)


#slide[
  #place(bottom, dy: -1.5em)[
    #text("Group 7", size: 25pt)
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
      node-stroke: 1pt,
      node-inset: 10pt,
      node-corner-radius: 1mm,
      edge-stroke: 2pt,
      spacing: 1em,
      {
        node((5cm, 5cm), [Middleware], name: <middleware>, width: 65mm)
        node((11cm, 5cm), stroke: none, align(left, text(
          [Authentication, \ Rate Limiting],
          size: 15pt,
          fill: gray.darken(30%),
        )))
        node((5cm, 0cm), [File Manager], name: <manager>, width: 65mm)
        node(
          enclose: ((10.5cm, 1.7cm), (15.5cm, -1.7cm)),
          place(top + left, text("Cache Layer", size: 20pt), dy: -1.2cm, dx: -.45cm),
          stroke: (dash: "dashed"),
        )
        node((13cm, 1cm), [File], name: <filecache>, width: 50mm)
        node((13cm, -1cm), [Metadata], name: <metadatacache>, width: 50mm)
        node(
          (22cm, -2cm),
          "Database",
          width: 60mm,
          height: 20mm,
          shape: shapes.cylinder.with(rings: 10%),
          name: <database>,
        )
        node(
          (22cm, 1.8cm),
          "File System",
          width: 60mm,
          height: 20mm,
          shape: shapes.cylinder.with(rings: (7%, 14%)),
          name: <disk>,
        )

        edge(<middleware.south>, "-|>", <manager.north>, label: "")
        edge(<manager.east>, <filecache.west>, "-|>")
        edge(<manager.east>, "-|>", <metadatacache.west>)

        edge(<filecache.east>, "-|>", <disk.west>)
        edge(<metadatacache.east>, "-|>", <database.west>)
      },
    )
  ]

]

#show: appendix

#slide[

  #place(center + horizon)[
    #rotate(-45deg)[
      #text("CLASSIFIED", fill: gray.lighten(80%), size: 80pt, font: "Inter Display", tracking: 6mm)
    ]
  ]

  #place(top + right)[
    #text([*CONFIDENTIAL -- DO NOT SHARE*], size: 14pt, weight: "thin")
  ]


  *Notes*
  #set text(size: 17pt)

  - crabdrive should behave similarly to other Cloud Platform providers (Google Drive, OneDrive, etc.)
  - We use *`axum`* or *`arctix`* as a server
  - The filetree is stored inside a SQL Database
]
