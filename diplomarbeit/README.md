
# The HTL Diploma Thesis

## Latex Template
The original template is in ./original_latex_template_htlinn, or it can be downloaded from the [diploma thesis moodle course](https://moodle2.htlinn.ac.at/course/view.php?id=399). In ./latex_template_htlinn is a modified version, to be used with the htldoc tool.

## How to use it with htldoc

TODO

Documentation for all the options can be found in [./options.md](./options.md)


## structure of the original_latex_template_htlinn
- content
    - abstract.tex              Hier kommen das deutsche und englische Abstract rein
    - einleitung.tex            Grundsätzliches zur gesamten Arbeit, auch indiv. Aufteilung der Arbeiten
    - latex_beispiele.tex       Grundsätzliche Latex-Funktionalität wird hier gezeigt
- figures
    - bsp.png
    - htl-logo2.png
    - htl-logo.png
- main.tex                      Hier läuft alles zusammen. Dort ist auch Thema, Betreuer, Abteilung, Jahr usw. einzutragen
- references.bib                Alle Zitate befinden sich hier. In latex_beispiele wird gezeigt wie man diese einbindet
- sourcecode
    - First.java
- template
    - affirmation.tex           Eidesstattliche Erklärung
    - listing_format.tex        Wie soll der Quellcode formatiert werden? Anwendung dazu in content/latex_beispiele
    - lock_flag.tex             Falls ein Sperrvermerk gemacht werden soll, dieses file einblenden bzw. ausblenden (in main.tex)
    - main_settings.tex         Grundsätzliches zum Basislayout. Hier sollte man wenig ändern müssen
    - mycommands.tex            Bestimmte Befehle werden hier überschrieben für einheitliches Layout. Hier sollte man wenig ändern müssen
    - pdf_settings.tex          Parameter für die PDF-Generierung. Hier sollte man wenig ändern müssen
    - preamble.tex              Hier kommen die ganzen imports hin
    - itle_thesis_htlinn.tex    Das Titelblatt, hier werden die Infos von main.tex eingebaut
    - typographic_settings.tex  Hier sollte man wenig ändern müssen

