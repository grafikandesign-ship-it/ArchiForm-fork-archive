# Brief del gruppo Architettura — ArchiForm

Stato: **inventario con decisioni di ambito concordate; implementazione da definire oggetto per oggetto**. Riferimento osservato: ARCHLine.XP 2021 Professional installato in locale, scheda **Architettura**. Le funzioni sotto riportate descrivono i menu e i comandi osservati, non un obiettivo di parità funzionale né una specifica da copiare. La terminologia, l'interfaccia e le icone di ArchiForm restano originali.

## Obiettivo di ArchiForm

Portare rapidamente una pianta verificabile a un modello architettonico parametrico, navigabile e trasferibile via IFC4. Il flusso principale è: livelli e riferimenti → muri → aperture → solai e coperture → scale e ringhiere → vani → controllo in 3D → tavole. La modellazione libera e gli arredi restano una fase successiva. Un comando deve essere disponibile anche nel motore/CLI/MCP, non soltanto nella GUI.

## Due flussi per l'esistente e la visualizzazione

ArchiForm distingue la modellazione BIM dalla costruzione di geometrie per visualizzazione e render. Un volume libero è utile per ricalcare una forma irregolare da un PDF calibrato, ma da solo non ospita porte/finestre e non viene esportato in IFC. Quando il volume deve avere aperture o passare a un modello BIM, il flusso concordato è **Volume di visualizzazione → Converti il volume in muri BIM**. La conversione crea segmenti di muro lungo il contorno, li collega ai comandi esistenti per porte e finestre e sostituisce il volume nella stessa operazione annullabile. I muri così generati vengono esportati in IFC4 con le relazioni di apertura e riempimento.

La conversione iniziale applica un tipo di muro a tutti i lati; l'utente può poi assegnare tipi diversi ai singoli segmenti dove cambiano gli spessori. Questo mantiene le aperture e l'export BIM senza introdurre una geometria muro personalizzata non ospitante. Il volume non convertito resta adatto alla resa 3D e non entra nell'IFC. L'export diretto della geometria di visualizzazione in OBJ/GLB è un'esigenza separata, da valutare dopo il primo collaudo del flusso aperture → IFC.

## Inventario della scheda Architettura di ARCHLine.XP 2021

| Area | Funzioni osservate nell'interfaccia | Lettura per ArchiForm |
|---|---|---|
| Proprietà | Proprietà dell'oggetto, stili e impostazioni | Editor specifico per ogni oggetto; distinguere istanza e tipo, con anteprima e applicazione esplicita. |
| Muri | Segmento, arco, da linee DWG, rettangolo, spline, da polilinea, da profilo frontale; connessioni; nicchia, profilo, lato, strati e isolamento | Strumenti distinti per geometria di tracciamento, composizione stratificata, giunzioni e modifica. |
| Porte | Catalogo di tipologie; inserimento in un punto o tra due punti, oggetto 3D libero; area aperture, identificazione, livello di dettaglio, creazione tipo | Porta ospitata nel muro con posizione, misure e quattro versi verificabili; libreria e editor del tipo. |
| Finestre | Come porte; lucernario e finestra d'angolo | Apertura ospitata, quota davanzale, simbolo in pianta e varianti speciali. |
| Vetrata continua | Disegno della geometria su faccia del muro | Elemento dedicato solo se i casi di facciata lo richiedono; altrimenti catalogo di componenti. |
| Colonne e travi | Inserimento di elementi architettonici con cataloghi di sezioni | Comandi esistenti in ArchiForm sotto Struttura; valutare accesso rapido anche da Architettura. |
| Solai | Contorno chiuso, automatico dai muri, inclinato, foro, rientranza/sporgenza, fascia, plinti, modifica profilo | Prima contorno, spessore, quota e fori; automatismi dai muri in seguito. I plinti restano in Struttura. |
| Tetti | Da polilinea, automatico dai muri, forme predefinite, falda singola, tre punti, sezione, abbaino, gronda, rilievo e strumenti di trave | Partire da falde e fori robusti; poi forme guidate e dettagli. |
| Controsoffitti | Lisci o a pannelli, manuali o derivati dai vani | Contorno/quota e associazione al vano; pannellatura in una fase ulteriore. |
| Scale | Schemi diritti, U, L, spirale; per rampe o gradini; modifica lato e rappresentazione; foro nel solaio | Numero gradini, verso di salita, pianerottoli e foro devono risultare coerenti in pianta, 3D e IFC. |
| Rampe | Dritte, ad arco, per ingombro; modifica lato/profilo e foro nel solaio | Quote iniziale/finale, pendenza, larghezza e raccordo con solai. |
| Ringhiere | Disegno libero o su scala/rampa, layout, modifica percorso, divisione | Ospitalità e aggiornamento quando cambia scala/rampa. |
| Vani/RoomBook | Vano da muri o polilinea, generazione totale, muro fittizio, esterni, partizioni, aggiornamento, colori, riepiloghi e liste | Vano derivato e aggiornabile; area/volume, nome e destinazione; abachi nella Presentazione. |
| Rilievo diretto | Vano da misure, triangolazione, punti ausiliari, connessione vani, generazione muri, dati | Flusso di ricostruzione dell'esistente, utile ma separato dalla costruzione quotidiana. |
| Terreno e Google Maps | Terreno da punti/curve, sbancamento, strada, zone, quote e discontinuità; dati geografici | Terreno semplice incluso nel gruppo Architettura, con priorità bassa. Senza rilievo topografico, le quote restano approssimate e dichiarate tali. |
| Volume edificio | Massa, spingi/tira, muri/solai per piano e vetrate da massa | ArchiForm ha già massa concettuale e conversioni principali. |
| Nuvola di punti | Importazione, taglio, intervalli di vista, ritaglio verticale, aggiornamento della pianta, supporto al tracciamento | Da valutare quando esiste un rilievo scanner; non blocca il gruppo Architettura attuale. |

L'inventario riguarda i comandi **visibili nella versione locale 2021**. Non sono state analizzate tutte le finestre di dettaglio, ogni voce secondaria del menu Connetti, tutti i preset del catalogo né il comportamento completo di ciascun comando. La documentazione ufficiale integra la lettura dei menu, ma le pagine aggiornate in anni successivi non provano che ogni dettaglio fosse presente nella versione 2021.

## Stato attuale verificato in ArchiForm

| Ambito | Stato | Riscontro nel repository / lavoro da verificare |
|---|---|---|
| Muri | Presente, da rifinire | Muri rettilinei e ad arco, tipi stratificati, linea di ubicazione, giunzioni, fasi, editor specifico con anteprima. Verificare disegno/modifica in tutti i versi e gestione dei muri esistenti irregolari. |
| Porte e finestre | Presente, da rifinire | Aperture ospitate e componenti parametrici; verificare editor dedicato, simboli, verso dopo l'inserimento, varianti speciali. |
| Solai, controsoffitti, tetti | Presente, da estendere | Categorie e comandi di creazione, contorni e fori ove previsti. Valutare gli automatismi dai muri, l'editor specifico e i casi complessi. |
| Scale, rampe, ringhiere | Presente, da verificare | Categorie, strumenti e comandi; il collaudo dell'oggetto scala deve controllare inizio, freccia e numerazione, prima di applicarlo a Bergem. |
| Vani | Presente, da rifinire | Categoria Room e generazione dai muri; verificare aggiornamento, parametri e collegamento alle finiture. |
| Masse e volumi per render | Presente, da estendere | Massa concettuale trasparente; volume opaco tracciabile per visualizzazione; conversione sostitutiva in segmenti di muri BIM per porte/finestre e IFC4. Verificare l'assegnazione di tipi diversi per i lati con spessori variabili. |
| Colonne, travi, fondazioni | Presente in Struttura | Evitare duplicazione del modello; eventualmente solo scorciatoie nella scheda Architettura. |
| Disegni di riferimento | Presente | Import CAD e riferimenti immagine/PDF fanno parte del flusso esistente; verificarne l'uso nel tracciamento dell'esistente. |
| Terreno, rilievo, nuvole di punti, vetrata continua nativa | Lacune o ambiti non ancora nativi | `Category` non definisce terreno, rilievo, nuvola di punti o curtain wall. Richiedono una scelta di modello dati prima di introdurre comandi UI. |

Questa tabella descrive **presenza di infrastruttura**, non certifica completezza o assenza di difetti. Il codice consultato è in `crates/cc_app/src/ribbon.rs`, `crates/cc_core/src/types.rs`, `crates/cc_engine/src/commands/create.rs` e nei moduli specialistici citati in `AGENTS.md`.

## Proposta di sequenza per il gruppo Architettura

1. **Regole comuni degli oggetti.** Prendere l'editor del muro già creato come riferimento da verificare, non come schema rigido. Per ogni oggetto definire i controlli realmente necessari, l'anteprima utile e la distinzione fra istanza e tipo. L'accesso dal clic destro e dal pannello Proprietà deve restare coerente.
2. **Involucro e aperture.** Completare muri, connessioni, stratigrafie, livelli, porte e finestre; preset frequenti e comportamento di selezione/modifica. Verificare la corrispondenza fra pianta, 3D e IFC.
3. **Elementi orizzontali e circolazione.** Solai e fori, controsoffitti, tetti, scale, rampe e ringhiere. Per le scale verificare in modo mirato verso di salita, gradino iniziale, numerazione e riferimenti ai piani.
4. **Vani e controllo del modello.** Aggiornamento automatico di area/volume, zone e abachi; controlli di elementi non ospitati e livelli incoerenti. Le tavole appartengono alla successiva rifinitura della Presentazione, ma devono poter leggere il modello.
5. **Terreno semplice, a bassa priorità.** Includere nel completamento di Architettura un terreno editabile con contorno, quota e punti/pendenza essenziali, senza presupporre un rilievo topografico. Sbancamenti, strade, nuvole di punti e integrazione geografica richiedono una successiva valutazione separata.
6. **Esistente irregolare e render.** Tracciare volumi liberi su riferimenti calibrati; quando servono aperture o IFC, convertirli in muri BIM segmentati. OBJ/GLB diretto per forme libere resta da progettare dopo aver verificato il flusso di conversione.

### Criteri per considerare chiuso il gruppo

- Il committente costruisce e prova **un oggetto alla volta** nella versione portatile Windows. Ogni ciclo comprende inserimento, selezione, modifica, salvataggio/riapertura e riscontro visivo in pianta e 3D; i problemi emersi guidano il ciclo successivo.
- Per ciascun oggetto, gli edit passano anche da comandi del motore, senza dipendere esclusivamente dalla GUI.
- Bergem potrà essere usato successivamente come caso reale, con distinzione fra strutture esistenti, geometrie stimate e discordanze documentate; non è il primo collaudo.
- Porte, finestre, scale, solai e tetto sono editabili dopo l'inserimento; le modifiche aggiornano pianta, 3D e IFC4 senza perdere l'identità degli elementi.
- I volumi lunghi e sottili usati come muri si convertono in singoli muri BIM sul loro asse, con spessore scelto in base alla massa; cliccando con Porta o Finestra su una massa compatibile, ArchiForm la converte e inserisce direttamente l'apertura. Per masse di edificio, la conversione crea muri perimetrali. Il volume sorgente viene rimosso nello stesso flusso annullabile; l'IFC4 contiene muri, aperture e riempimenti coerenti.
- Il progetto si salva e riapre; annullamento e copia del tipo sono prevedibili; gli errori non danneggiano il documento.
- La versione rimane fissa durante lo sviluppo; si prova solo l'eseguibile portatile Windows. Il rilascio multipiattaforma avviene dopo l'accettazione del gruppo, come definito in `docs/development-workflow.md`.

## Decisioni concordate e prossimi dettagli

1. **Confine del gruppo:** il terreno semplice rientra in Architettura, ma viene dopo gli oggetti edilizi principali.
2. **Editor:** quello del muro è un punto di partenza; struttura e controlli vanno verificati separatamente per porte, finestre, scale, tetti e gli altri oggetti.
3. **Collaudo:** l'utente costruisce e testa oggetto per oggetto. Il progetto Bergem non è il primo banco di prova.
4. **Da definire prima di ogni implementazione:** comportamento minimo dell'oggetto, proprietà editabili, anteprima necessaria e criterio di accettazione dell'oggetto stesso.

Il primo ciclo è descritto in [`architettura-muro.md`](architettura-muro.md).

## Fonti

- Osservazione diretta di ARCHLine.XP 2021 Professional installato localmente, scheda Architettura e menu dei comandi (2 novembre 2026).
- [ARCHLine.XP, Architectural tools](https://www.archlinexp.com/archline-xp/architectural-benefits/architectural-tools).
- [ARCHLine.XP, Architecture Level 1](https://www.archlinexp.com/education/tutorial-videos/archline-xp-architecture-level-1).
- [ARCHLine.XP, Placing doors and windows](https://help.archlinexp.com/hc/en-us/articles/4407066166673-Placing-doors-windows).
- [ARCHLine.XP, Storey level bounded parameters](https://help.archlinexp.com/hc/en-us/articles/360013693578-Storey-level-bounded-parameters).
- [ARCHLine.XP, How to use layers](https://help.archlinexp.com/hc/en-us/articles/360019730417-How-to-use-layers).
- [ARCHLine.XP, Create terrain](https://help.archlinexp.com/hc/en-us/articles/360020249658-Create-terrain).
- [ARCHLine.XP 2021 release information](https://help.archlinexp.com/hc/en-us/articles/360019172978-Release-Information).
