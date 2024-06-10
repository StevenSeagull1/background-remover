een hele simpele half afgemaakte background remover in rust:

overlap:
in overlap worden 2 fotos met elkaar vergeleken. 1 waarin er mensen zijn 1 waarin het leeg is.
hierbij wordt het gene wat opelkaar lijkt weg gehaald en alles wat anders is blijft.

verhouding:
in verhouding gebeurt het net iets anders. hier worden ook de foto's vergeleken maar wordt er gekeken naar of de verhoduing erg verandert.
hier wordt het gemmidelde gepakt van de rgb waardes. dus een pixel met R30 G40 B50 heeft een gemmidelde van 40. als de pixel in de andere foto.
een compleet ander gemmidelde heeft wordt de pixel weg gehaald. op deze manier kun je ervoor zorgen dat als er een groot verschil is in belichting er niks veranderd aan het algoritme.


TODO: het probleem met overlap is nu nog dat belichting een te groot impact heeft en voor problemen kan veroorzaken. hiervoor is "verhouding" gemaakt het probleem hiermee is is dat
je dan het probleem krijgt dat hele lage waardes die donker zijn in de weg gaan zitten. hiervoor zou nu nog een oplossing voor bedacht moeten worden. een idee zou zijn om alle lage waardes te negeren.
