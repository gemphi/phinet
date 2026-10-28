# sys

```mermaid
graph TB
    MdFiles["persona *.md files<br/>stage/, code/"]
    Quests["quest folders<br/>slug/seed/relations.tsv"]
    BSrc["persona sources<br/>.b directories"]
    Phiecs["phiecs<br/>Persona, World, Row, Kit"]
    Phient["phient::relate<br/>read_quests, scopes, Casting<br/>code::bcpl::read_coarse"]
    EyeFlow["phiecs::kit::eye::Flow<br/>parallax rows"]
    Phiclm["phiclm::judge"]
    ClmLisp["phiclm::lisp<br/>Machine, Value, read, 39 core primitives<br/>set: the relation algebra"]
    Users["arena::ecs<br/>examples circus, race, sets, lisp, think, writer"]

    subgraph sys["phiano::sys"]
        SysRs["sys.rs<br/>re-exports Persona, World, Row, Kit, Casting"]
        Agent["agent.rs: trait Agent<br/>tell, watch, ask, told, kudos"]
        Being["being.rs: Being<br/>new, from_quests, from_file<br/>find, persona_files"]
        Eye["eye.rs<br/>see(who, flow, angle)"]
        Circus["circus.rs: Turn<br/>line, talk, render"]
        Race["race.rs: Race, Standing<br/>engage, standings, winner, table"]
        SetRs["set.rs<br/>held, think to Thought, render"]
        Trace["trace.rs: Scope<br/>says, next, trace<br/>scopes_of, write, summary"]
        LispRs["lisp.rs: Sys, the Lisp's host<br/>next, says, scopes, held, ask, tell, told, beings"]
    end

    Phiecs --> SysRs
    Phient --> SysRs
    SysRs --> Agent
    MdFiles -->|"Persona::from_md"| Being
    Quests -->|"read_quests rows"| Being
    BSrc -->|"bcpl::read_coarse"| Being
    Being -->|"impl Agent"| Agent
    Agent -->|"tell rows into World"| Phiecs
    Phiecs -->|"Kit rows to watch"| Agent
    EyeFlow -->|"flow.rows sight"| Eye
    Eye -->|"tell"| Agent
    Being --> Circus
    Circus -->|"heard row told to listener"| Being
    Being -->|"told, words, kinds, kudos"| Race
    Being -->|"world edges under kind"| SetRs
    SetRs -->|"union, meet, compose"| ClmLisp
    Phient -->|"scopes(path)"| Trace
    Trace -->|"pairs()"| SetRs
    LispRs -->|"impl Host"| ClmLisp
    LispRs -->|"held, ask, tell"| Being
    LispRs -->|"next, says"| Trace
    Phiclm -->|"judged rows"| ClmLisp
    Race --> Users
    Circus --> Users
    LispRs --> Users
```
