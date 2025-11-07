# Entwicklungsreihenfolge

## Phase 1: Terminal-Setup (main.rs)

- Beginne mit Terminal initialisierung und einfachem Event-Loop
- Dadurch erhalten wir eine funktionierende Basis, auf der wir aufbauen können
- Können sofort testen, ob ratatui korrekt funktioniert und ob richtig gerendert wird

## Phase 2: Basis-UI (ui.rs + app.rs)

- Implementiere einfache App-Struktur und rudimentäres Rendering
- Zunächst nur statischer Text, um Layout-System zu verstehen
- Das UI-Modul sollte ein einfaches Layout mit drei Bereichen:
  - Hauptbereich
  - Statusleiste
  - Command-line
- Zusätzlich soll ein permanenter File-Explorer auf der linken Seite angezeigt werden
- Eine Terminal-Integration soll ebenfalls angezeigt werden

## Phase 3: Text-Buffer (buffer/)

- Herzstück des Editors
- Grundlegende Operationen:
  - Text speichern
  - Zeilen verwalten
  - Cursor bewegen
- Kritischer Schritt, da alle anderen Module hierauf aufbauen werden

## Phase 4: Modi-System (editor/)

- Implementiere verschiedene Modi und die Logik zum Wechseln zwischen ihnen
- Beginne mit Normal- und Inster-Modus für grundlegende Funktionalität

## Phase 5: Input-Handling (input/)

- Tastatureingabe mit Aktionen verbinden
- Zunächst Bewegungsbefehle mit Pfeiltasten (oder klassisch h, j, k, l)
- Texteingabe im Insert-Modus

## Phase 6: Commands (commands/)

- Kommandos für jeden Modus
- Mit den wichtigsten Befehlen anfangen
- Schrittweise erweitern...

