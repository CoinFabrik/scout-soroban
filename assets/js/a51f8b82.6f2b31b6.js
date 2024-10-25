"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[1098],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>f});var a=n(9496);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var s=a.createContext({}),c=function(e){var t=a.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},u=function(e){var t=c(e.components);return a.createElement(s.Provider,{value:t},e.children)},m="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),m=c(n),d=r,f=m["".concat(s,".").concat(d)]||m[d]||p[d]||i;return n?a.createElement(f,o(o({ref:t},u),{},{components:n})):a.createElement(f,o({ref:t},u))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,o=new Array(i);o[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[m]="string"==typeof e?e:r,o[1]=l;for(var c=2;c<i;c++)o[c]=n[c];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},5162:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>p,frontMatter:()=>i,metadata:()=>l,toc:()=>c});var a=n(2564),r=(n(9496),n(9613));const i={},o="Insufficiently random values",l={unversionedId:"detectors/insufficiently-random-values",id:"detectors/insufficiently-random-values",title:"Insufficiently random values",description:"Description",source:"@site/docs/detectors/5-insufficiently-random-values.md",sourceDirName:"detectors",slug:"/detectors/insufficiently-random-values",permalink:"/scout-soroban/docs/detectors/insufficiently-random-values",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/5-insufficiently-random-values.md",tags:[],version:"current",sidebarPosition:5,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Overflow-check",permalink:"/scout-soroban/docs/detectors/overflow-check"},next:{title:"Unprotected update current contract wasm",permalink:"/scout-soroban/docs/detectors/unprotected-update-current-contract-wasm"}},s={},c=[{value:"Description",id:"description",level:2},{value:"Why is this bad?",id:"why-is-this-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2},{value:"References",id:"references",level:2}],u={toc:c},m="wrapper";function p(e){let{components:t,...n}=e;return(0,r.kt)(m,(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"insufficiently-random-values"},"Insufficiently random values"),(0,r.kt)("h2",{id:"description"},"Description"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Category: ",(0,r.kt)("inlineCode",{parentName:"li"},"Block attributes")),(0,r.kt)("li",{parentName:"ul"},"Severity: ",(0,r.kt)("inlineCode",{parentName:"li"},"Critical")),(0,r.kt)("li",{parentName:"ul"},"Detector: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/insufficiently-random-values"},(0,r.kt)("inlineCode",{parentName:"a"},"insufficiently-random-values"))),(0,r.kt)("li",{parentName:"ul"},"Test Cases: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1"},(0,r.kt)("inlineCode",{parentName:"a"},"insufficiently-random-values-1"))," ")),(0,r.kt)("p",null,"Using block attributes like ",(0,r.kt)("inlineCode",{parentName:"p"},"timestamp")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"sequence")," for random number generation in Soroban smart contracts is not recommended due to the predictability of these values. Block attributes are publicly visible and deterministic, making it easy for malicious actors to anticipate their values and manipulate outcomes to their advantage. It's important to use a source that is both unpredictable and external to the blockchain environment, reducing the potential for malicious exploitation."),(0,r.kt)("h2",{id:"why-is-this-bad"},"Why is this bad?"),(0,r.kt)("p",null,"Using ",(0,r.kt)("inlineCode",{parentName:"p"},"ledger().timestamp()")," is not recommended because it could be potentially manipulated by validator, which might lead to potential problems. On the other hand, ",(0,r.kt)("inlineCode",{parentName:"p"},"ledger().sequence()")," is publicly available. An attacker could predict the random number to be generated to manipulate the code and perform an attack on the contract."),(0,r.kt)("h2",{id:"issue-example"},"Issue example"),(0,r.kt)("p",null,"Consider the following ",(0,r.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"pub fn generate_random_value_timestamp(env: Env, max_val: u64) -> Result<u64, Error> {\n        if max_val == 0 {\n            Err(Error::MaxValZero)\n        } else {\n            let val = env.ledger().timestamp() % max_val;\n            Ok(val)\n        }\n    }\n    \n    pub fn generate_random_value_sequence(env: Env, max_val: u32) -> Result<u32, Error> {\n        if max_val == 0 {\n            Err(Error::MaxValZero)\n        } else {\n            let val = env.ledger().sequence() % max_val;\n            Ok(val)\n        }\n    }\n    \n")),(0,r.kt)("p",null,"The issue lies in these functions use of blockchain-provided data like block timestamp and sequence number for pseudo-random number generation. This reliance on predictable blockchain data makes the generated values susceptible to manipulation by attackers."),(0,r.kt)("p",null,"The code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1/vulnerable-example"},"here"),"."),(0,r.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,r.kt)("p",null,"Avoid using block attributes like ",(0,r.kt)("inlineCode",{parentName:"p"},"timestamp")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"sequence")," for randomness generation, and consider using PRNG instead."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"\n  pub fn generate_random_value(env: Env, max_val: u64) -> Result<u64, Error> {\n        if max_val == 0 {\n            Err(Error::MaxValZero)\n        } else {\n            let val = env.prng().gen_range(0..max_val);\n            Ok(val)\n        }\n    }\n        \n")),(0,r.kt)("p",null,"The remediated code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1/remediated-example"},"here"),"."),(0,r.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,r.kt)("p",null,"Checks the usage of ",(0,r.kt)("inlineCode",{parentName:"p"},"ledger().timestamp()")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"ledger().sequence()")," for generation of random numbers."),(0,r.kt)("h2",{id:"references"},"References"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://dasp.co/#item-6"},"https://dasp.co/#item-6")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://blog.sigmaprime.io/solidity-security.html#SP-6"},"https://blog.sigmaprime.io/solidity-security.html#SP-6")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://swcregistry.io/docs/SWC-120"},"SWC-120")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://swcregistry.io/docs/SWC-116"},"SWC-116")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://ethernaut.openzeppelin.com/level/0x4dF32584890A0026e56f7535d0f2C6486753624f"},"Ethernaut: Coinflip")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#weak-PRNG"},"Slither: Weak PRNG")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#block-timestamp"},"Slither: Dangerous usage of block.timestamp"))))}p.isMDXComponent=!0}}]);