// Headless probe of the CaseForge v5 engine: loads the engine functions out of index.html
// (no DOM, no UI) and replays the behaviours recorded in research/findings.md.
// Run: node research/prototypes/caseforge-v5/probe.mjs
import { readFileSync } from "node:fs";
import vm from "node:vm";

const html = readFileSync(new URL("./index.html", import.meta.url), "utf8");
const slice = (from, to) => {
  const a = html.indexOf(from), b = html.indexOf(to, a);
  if (a < 0 || b < 0) throw new Error(`marker not found: ${from} .. ${to}`);
  return html.slice(a, b);
};
const engine = [
  slice('var $=function', "function toast("),          // helpers, intents, slots, gate, patterns, split, seed, index, learning
  slice("function solveSub(", "function singleOut("),  // solve one subproblem, merge
  slice("function headPred(", "function mineChats("),  // article -> atoms
].join("\n");
const stubs = `
var window={matchMedia:function(){return{matches:false}},addEventListener:function(){},innerWidth:1,innerHeight:1};
var document={querySelector:function(){return null},querySelectorAll:function(){return[]},addEventListener:function(){},hidden:false};
function renderLog(){}function renderHeader(){}function renderHealth(){}
`;
const tests = `
function reset(){var sd=seed();S.cases=sd.cases;S.records=sd.records;S.traces=sd.traces;reindex()}
function ask(t){var seg=segment(t);var solved=seg.groups.map(function(g){return solveSub(g,[],t)});
 return{split:seg.info.mode,text:solved.length>1?mergeSubs(solved,seg.info).text:solved[0].text,
  subs:solved.map(function(s){return{intent:s.intent,mode:s.mode,pattern:s.pattern,atom:s.atom,subject:s.slots.subject}})}}
function show(label,q){var r=ask(q);out((label?"\\n"+label+"\\n":"")+"  Q: "+q+"\\n  "+r.subs.map(function(s){return s.intent+" -> "+s.mode+" (pattern "+(s.pattern||"-")+", atom "+(s.atom||"-")+", subject "+s.subject+")"}).join("\\n  ")+"\\n  A: "+(r.text||"<cold start>"))}
reset();
out("== One pattern, six domains (gated-request, never edited) ==");
["I want a refund for order 7741, it arrived 5 days ago and is still sealed",
 "my laptop is faulty, bought it 6 months ago, warranty claim T-204",
 "reimburse my expense EXP-77 of $120, receipt attached",
 "leave request APP-9 with 20 days notice, coverage arranged",
 "enable advanced features on ACC-3, we are on pro and verified",
 "upgrade ACC-3, 4 seats"].forEach(function(q){show("",q)});

out("\\n== New domain from an article: cancellation ==");
var Q="I want to cancel my subscription SUB-12, I bought it 5 days ago.";
show("before mining",Q);
var mined=mineArticle("## Subscription cancellation refunds\\n\\nMembers can end a recurring plan at any time from the billing console, and access always continues to the end of the paid period. Cancellations are approved within 14 days of purchase. Once activated, a pause is available instead. Our finance team reviews edge cases monthly and publishes aggregate statistics in the transparency report.");
out("\\nmined "+mined.length+" atoms: "+mined.map(function(x){return x.predicate+"/"+x.object+" "+JSON.stringify(x.conditions)}).join("; "));
var pay=["Cancellation approved for {subject} — access continues to the end of the paid period and no further charges apply.","That plan is already activated, so a pause is available instead — up to 3 months at $0 with your data preserved."];
mined.forEach(function(x,i){createCase({kind:"atom",predicate:"cancel",object:"subscription",domain:"subscriptions",conditions:x.conditions,slots:x.slots,payload:pay[i],result:x.result,priority:x.priority,trust:x.trust,source:"mined"})});
show("after mining (predicate hand-corrected refund -> cancel, payloads rewritten)",Q);
show("other branch, same pattern","cancel my plan SUB-12, it's already activated and I bought it 40 days ago");

show("unknown topic behind a greeting is refused, not greeted","hi, my sourdough starter smells like acetone, is it dead?");

out("\\n== Defects ==");
show("D1 slots are global, not per subproblem","cancel my subscription SUB-12 bought 5 days ago, also where is my order 5150?");
show("D2 negation is ignored by the slot extractor","my laptop stopped working, bought it 6 months ago, never dropped, warranty claim T-204");
show("D3 atoms of another object compete (order vs subscription)","refund order 7741, it arrived 45 days ago, still sealed");
reset();
var modes={},grid={days:[5,45],packaging:["original","opened"],activated:[true,false],term:[6,24],misuse:[true,false],seats:[0,3],amount:[100,900],receipt:[true,false],notice:[5,20],coverage:[true,false],tier:["pro","basic"],verified:[true,false]},p=byId("pat1");
["refund","warranty","upgrade","reimburse","leave","enable"].forEach(function(it){
 var used={};atomsFor(PRED[it]).forEach(function(a){a.conditions.forEach(function(c){used[c.slot]=1})});var u=Object.keys(used);
 (function rec(i,sl){if(i===u.length){var m=interpret(p,bindRoles(p,it,sl)).mode;modes[m]=(modes[m]||0)+1;return}
  [undefined].concat(grid[u[i]]).forEach(function(v){var s=Object.assign({},sl);if(v!==undefined)s[u[i]]=v;rec(i+1,s)})})(0,{subject:"X"});});
out("\\nD4 the skeleton's offer step never fires; reply modes over every slot combination of the seed atoms:\\n  "+JSON.stringify(modes));
S.traces=[];["greeting","pricing","hours"].forEach(function(i){S.traces.push({intent:i,shape:"move",sig:"direct",domain:"general"})});
var cand=induce()[0],res;try{clone(SKEL[cand.shape]);res="ok"}catch(e){res="throws: "+e.message}
out("\\nD5 promoting an induced candidate built from cached-move traces\\n  candidate "+JSON.stringify(cand)+"\\n  promote "+res);
`;
vm.runInNewContext(`"use strict";${stubs}${engine}${tests}`, { out: (s) => console.log(s), performance });
