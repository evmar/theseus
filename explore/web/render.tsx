import dataRaw from "./data.json" with { type: "json" };
import * as preact from "preact";
import * as hooks from "preact/hooks";

import type { Blocks } from "../bindings/Blocks.js";
import type { Block } from "../bindings/Block.js";
import type { Instr } from "../bindings/Instr.js";
import type { Effect } from "../bindings/Effect.js";
import type { Expr } from "../bindings/Expr.js";
import type { Call } from "../bindings/Call.js";
import type { Var } from "../bindings/Var.js";

const data: Blocks = dataRaw as any;

const HoverContext = preact.createContext<any>(undefined);

function hex(n: number): string {
  return "0x" + n.toString(16);
}

function Call(props: Call) {
  const { op, args } = props;
  return (
    <span>
      ({op}
      {args.map((e) => (
        <>
          {" "}
          <Expr expr={e} />
        </>
      ))}
      )
    </span>
  );
}

function Var(props: Var) {
  const { hover, setHover } = hooks.useContext(HoverContext);
  const { reg, ver } = props;
  const id = `${reg}#${ver}`;
  const highlight = id === hover;
  return (
    <span class={highlight ? "highlight" : ""} onMouseOver={() => setHover(id)}>
      {reg}
      {ver == 0 ? "" : `#${ver}`}
    </span>
  );
}

function Expr(props: { expr: Expr }) {
  const { expr } = props;
  if ("Const" in expr) {
    return <span>{hex(expr.Const)}</span>;
  } else if ("Var" in expr) {
    return <Var {...expr.Var} />;
  } else if ("Call" in expr) {
    return <Call {...expr.Call} />;
  }
  throw new Error(expr);
}

function Eff(props: { eff: Effect }) {
  const { eff } = props;
  if ("Set" in eff) {
    const [x, y] = eff.Set;
    return (
      <span>
        <Expr expr={x} /> = <Expr expr={y} />
      </span>
    );
  } else if ("Call" in eff) {
    return <Call {...eff.Call} />;
  } else if ("Jmp" in eff) {
    const { cond, dsts } = eff.Jmp;
    return (
      <span>
        {<Call {...cond} />}{" "}
        {dsts.map((e) => (
          <>
            {" "}
            <Expr expr={e} />
          </>
        ))}
      </span>
    );
  }
  throw new Error(eff);
}

function Instr(props: { instr: Instr }) {
  const { addr, iced, eff } = props.instr;
  return (
    <div>
      <Eff eff={eff} />
    </div>
  );
  //  ; {hex(addr)} {iced}
}

function Block(props: { block: Block }) {
  const { block } = props;
  const addr = block.instrs[0]!.addr;
  return (
    <div class="block">
      {addr.toString(16)} (
      {block.params.map((v) => (
        <>
          {" "}
          <Var {...v} />
        </>
      ))}
      )
      {block.instrs.map((instr) => (
        <Instr instr={instr} />
      ))}
      {block.links.map((l) => (
        <div>
          &rarr; {hex(l.addr)}{" "}
          {l.params.map(([src, dst]) => (
            <span>
              {" "}
              <Var {...src} />:<Var {...dst} />
            </span>
          ))}
        </div>
      ))}
    </div>
  );
}

function Body() {
  const [hover, setHover] = hooks.useState(undefined);
  const hoverState = hooks.useMemo(() => {
    return { hover, setHover };
  }, [hover]);

  return (
    <HoverContext.Provider value={hoverState}>
      <main>
        {data.vec.map((block) => (
          <div>
            <Block block={block} />
          </div>
        ))}
      </main>
    </HoverContext.Provider>
  );
  return <div>hello</div>;
}

preact.render(<Body />, document.body);
