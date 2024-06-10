Een hele simpele half afgemaakte background remover in Rust.
Voor als je geen Rust begrijpt, raad ik aan om hier even te kijken: [Rust Book](https://doc.rust-lang.org/book/)

De bedoeling is om een algoritme te maken om achtergronden weg te halen. Hier zijn twee verschillende manieren uitgekomen met elk andere resultaten. Hij is nog niet af, maar ik ben zover gekomen:

De lege foto die ik gebruik in de twee methodes voor de vergelijking:<br>
<img src="https://github.com/StevenSeagull1/background-remover/assets/87282545/62ce727d-7be8-4caf-b5cd-1421689f4a90" alt="leeg01" style="width: 50%; height: auto;">

Overlap:
Bij overlap worden twee foto’s met elkaar vergeleken: één waarin er mensen zijn en één waarin het leeg is. Hierbij wordt wat op elkaar lijkt weggehaald en alles wat anders is blijft. Hier is een voorbeeld:<br>
<img src="https://github.com/StevenSeagull1/background-remover/assets/87282545/697ca923-4000-4f1a-82dc-049290d022e0" alt="selfie02" style="width: 50%; height: auto;"> <img src="https://github.com/StevenSeagull1/background-remover/assets/87282545/9a55120a-9d0e-4438-869d-47e65fc6c47d" alt="poging3" style="width: 50%; height: auto;">

Verhouding:
Bij verhouding gebeurt het net iets anders. Hier worden ook de foto's vergeleken, maar er wordt gekeken naar of de verhouding erg verandert. Hier wordt het gemiddelde gepakt van de RGB-waardes. Dus een pixel met R30, G40, B50 heeft een gemiddelde van 40. Als de pixel in de andere foto een compleet ander gemiddelde heeft, wordt de pixel weggehaald. Op deze manier kun je ervoor zorgen dat als er een groot verschil is in belichting, er niks verandert aan het algoritme. Hier is een voorbeeld:<br>
<img src="https://github.com/StevenSeagull1/background-remover/assets/87282545/697ca923-4000-4f1a-82dc-049290d022e0" alt="selfie02" style="width: 50%; height: auto;"> <img src="https://github.com/StevenSeagull1/background-remover/assets/87282545/7f765461-654a-41ca-a979-0cc570ddc27b" alt="poging4" style="width: 50%; height: auto;">

TODO:
Het probleem met overlap is nu nog dat belichting een te grote impact heeft en voor problemen kan zorgen. Hiervoor is "verhouding" gemaakt. Het probleem hiermee is dat je dan het probleem krijgt dat hele lage waardes die donker zijn in de weg gaan zitten. Hiervoor zou nu nog een oplossing bedacht moeten worden. Een idee zou zijn om alle lage waardes te negeren.
