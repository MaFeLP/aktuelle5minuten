#set document(author: "{{ author }}", title: "{{ title }}")
#set page(numbering: "1", number-align: center)
#set par(justify: true)
#set text(font: "Liberation Serif", lang: "de")
#show math.equation: set text(weight: 400)
#show heading: set text(font: "Liberation Sans")
#set heading(numbering: "1.1")
#align(center)[
  #block(text(font: "Liberation Sans", weight: 700, 1.75em, "{{ title }}"))
  #v(1em, weak: true)
  #block(
    text(
      font: "Liberation Sans",
      weight: 400,
      1.25em,
      [
        #datetime.today().display("[day].[month].[year]")
        #sym.dash.em
        #strong("{{ author }}")
      ],
    )
  )
]

// Begin content here