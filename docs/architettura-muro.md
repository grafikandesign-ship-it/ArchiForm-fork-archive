# Architettura — primo ciclo: muro

Stato: **scheda di lavoro per il collaudo dell'utente**, aggiornata dopo la verifica diretta di ARCHLine.XP 2021. Riferimento: `docs/brief-architettura.md`. Le funzioni di ARCHLine.XP sono un riferimento per valutare il flusso, non una specifica da riprodurre integralmente.

## Flusso osservato in ARCHLine.XP 2021

- Clic su **Muro**: entra nel disegno e sostituisce temporaneamente il pannello laterale con l'elenco dei set di muro, raggruppati per categorie. Il set selezionato mostra anche una piccola anteprima.
- Clic destro sull'icona **Muro** nel ribbon: menu con **Proprietà**, **SET**, selezione e altre voci generali. **Proprietà** apre la configurazione del tipo prima di disegnare; **SET** apre la gestione dei set.
- Nelle proprietà del muro la stessa anteprima passa fra **Vista 2D**, **Modello 3D** e **Sezione**. È presente un editor separato degli strati. Non tutti gli altri controlli della finestra sono necessari al primo ciclo di ArchiForm.

Nel repository ArchiForm i preset e le categorie esistevano già, ma si aprivano dal menu nella barra delle opzioni. Il flusso laterale e l'accesso alle proprietà prima del disegno sono oggetto della prima modifica.

## Risultato richiesto

Disegnare un muro in pianta, selezionarlo, modificarne istanza e tipo, vedere chiaramente l'effetto della modifica e ritrovare lo stesso muro dopo salvataggio e riapertura. Pianta, 3D e IFC4 devono descrivere lo stesso oggetto. La costruzione deve restare rapida, con proprietà avanzate accessibili quando servono.

## Concetti da tenere distinti

| Ambito | Proprietà | Effetto atteso |
|---|---|---|
| Istanza | Livello, estremi/tracciato, lunghezza risultante, linea di ubicazione, quota base, altezza o vincolo superiore, giunzioni, delimitazione vani, portanza, fase e layer | Cambia solo il muro selezionato. |
| Tipo | Nome, funzione (interno/esterno ecc.), strati ordinati, spessori e materiali | Cambia tutti i muri che usano quel tipo; **Salva come nuovo tipo** crea una variante indipendente e la assegna al muro selezionato. |
| Anteprima | Pianta con lato interno/esterno e linea di ubicazione; sezione degli strati con spessore totale; vista 3D semplificata quando utile | Mostra la bozza prima di applicarla, con i valori effettivi. |

## Stato verificato nel codice

- Il disegno supporta muro diritto e arco; **Spazio** cicla sei linee di ubicazione durante il tracciamento; **Maiuscolo** interviene nello snap direzionale. Sono presenti spessore in anteprima e quota temporanea.
- La finestra dedicata si apre per un muro selezionato e, dopo la prima modifica, anche per il tipo attivo prima del disegno. La bozza non modifica il progetto finché non si salva. Offre nome del tipo, altezza non vincolata, quota base, linea di ubicazione, giunzioni, portanza, delimitazione vani, funzione, strati/materiali e Salva come nuovo tipo.
- Il pannello Proprietà laterale espone anche il livello, il vincolo superiore e il relativo offset. Questi controlli **mancano nella finestra dedicata**.
- L'anteprima della finestra dedicata è la sezione degli strati. **Mancano una pianta e una vista 3D della bozza**. I cambiamenti diventano visibili nel progetto soltanto dopo Salva.
- Il motore dispone di `wall_create`, modifica dell'elemento e modifica/duplicazione del tipo. La geometria e le viste sono derivate dal modello.

Queste sono constatazioni dal codice, non risultati del collaudo visivo. In particolare, il comportamento dello snap con Maiuscolo e delle quote selezionate va verificato nell'eseguibile portatile.

## Primo ciclo di collaudo dell'utente

1. Attivare **Muro**: il catalogo laterale deve mostrare i tipi del progetto per funzione e i preset per categoria; il navigatore di progetto deve rimanere raggiungibile. Scegliere un tipo e poi un preset. Il preset scelto deve diventare un tipo modificabile del progetto.
2. Fare clic destro sull'icona **Muro** nel ribbon e aprire **Proprietà**. Modificare un valore, annullare e verificare che il tipo resti invariato; ripetere con **Salva come nuovo tipo** e verificare che il nuovo tipo diventi attivo per il disegno.
3. In un progetto di prova, disegnare due muri dello stesso tipo: uno orizzontale e uno verticale. Durante il disegno, usare **Maiuscolo** per bloccare la direzione e **Spazio** per cambiare la linea di ubicazione.
4. Selezionare un muro. Controllare che quota e riferimenti mostrati riguardino **quel muro**, e che i due estremi e le giunzioni siano riconoscibili.
5. Aprire la finestra dal clic destro sul muro. Cambiare altezza e quota base, poi **Annulla**: nessuna modifica deve restare nel progetto.
6. Riaprire, cambiare lo spessore di uno strato e premere **Salva**: entrambi i muri dello stesso tipo devono aggiornarsi; la sezione deve mostrare lo spessore totale corretto.
7. Riaprire e usare **Salva come nuovo tipo**: solo il muro selezionato deve assumere la nuova composizione.
8. Cambiare livello o vincolo superiore dal pannello laterale e controllare l'effetto in 3D. Salvare, chiudere e riaprire il progetto.

Per ogni passo, annotare il comportamento osservato, l'eventuale risultato atteso diverso e, se possibile, uno screenshot. Si corregge il problema concreto prima di aggiungere nuove opzioni.

## Interventi candidati dopo il primo riscontro

1. Riunire nella finestra dedicata i vincoli di istanza oggi dispersi fra finestra e pannello laterale, iniziando da livello e vincolo superiore.
2. Aggiungere un'anteprima in pianta della bozza che mostri linea di ubicazione, lato interno/esterno, spessore totale e lunghezza. Una vista 3D va aggiunta se aiuta davvero a leggere altezza, quota e stratigrafia.
3. Chiarire nell'interfaccia quali campi sono **Istanza** e quali sono **Tipo**, soprattutto prima di cambiare uno strato condiviso.
4. Verificare che i comandi del motore coprano ogni nuova modifica e che Annulla/Salva come nuovo tipo restino prevedibili.

## Criterio di chiusura dell'oggetto muro

L'utente riesce a completare i passaggi di collaudo senza incongruenze bloccanti. Dopo le correzioni, verifica personalmente la versione portatile Windows. Solo allora si passa all'oggetto successivo; la versione del programma rimane invariata fino alla chiusura dell'intero gruppo Architettura.
