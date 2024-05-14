"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[5788],{9613:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var r=n(9496);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),u=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},c=function(e){var t=u(e.components);return r.createElement(s.Provider,{value:t},e.children)},p="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,s=e.parentName,c=o(e,["components","mdxType","originalType","parentName"]),p=u(n),d=a,f=p["".concat(s,".").concat(d)]||p[d]||m[d]||i;return n?r.createElement(f,l(l({ref:t},c),{},{components:n})):r.createElement(f,l({ref:t},c))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,l=new Array(i);l[0]=d;var o={};for(var s in t)hasOwnProperty.call(t,s)&&(o[s]=t[s]);o.originalType=e,o[p]="string"==typeof e?e:a,l[1]=o;for(var u=2;u<i;u++)l[u]=n[u];return r.createElement.apply(null,l)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},6079:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>l,default:()=>m,frontMatter:()=>i,metadata:()=>o,toc:()=>u});var r=n(2564),a=(n(9496),n(9613));const i={},l="Insufficiently random values",o={unversionedId:"vulnerabilities/insufficiently-random-values",id:"vulnerabilities/insufficiently-random-values",title:"Insufficiently random values",description:"Description",source:"@site/docs/vulnerabilities/5-insufficiently-random-values.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/insufficiently-random-values",permalink:"/scout-soroban/docs/vulnerabilities/insufficiently-random-values",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/vulnerabilities/5-insufficiently-random-values.md",tags:[],version:"current",sidebarPosition:5,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Overflow check",permalink:"/scout-soroban/docs/vulnerabilities/overflow-check"},next:{title:"Unprotected update current contract wasm",permalink:"/scout-soroban/docs/vulnerabilities/unprotected-update-current-contract-wasm (copy)"}},s={},u=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2},{value:"References",id:"references",level:2}],c={toc:u},p="wrapper";function m(e){let{components:t,...n}=e;return(0,a.kt)(p,(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"insufficiently-random-values"},"Insufficiently random values"),(0,a.kt)("h2",{id:"description"},"Description"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,a.kt)("inlineCode",{parentName:"li"},"Block attributes")),(0,a.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,a.kt)("inlineCode",{parentName:"li"},"Critical")),(0,a.kt)("li",{parentName:"ul"},"Detectors: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/insufficiently-random-values"},(0,a.kt)("inlineCode",{parentName:"a"},"insufficiently-random-values"))),(0,a.kt)("li",{parentName:"ul"},"Test Cases: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1"},(0,a.kt)("inlineCode",{parentName:"a"},"insufficiently-random-values-1")))),(0,a.kt)("p",null,"Using block attributes like ",(0,a.kt)("inlineCode",{parentName:"p"},"timestamp")," or ",(0,a.kt)("inlineCode",{parentName:"p"},"sequence")," for random number generation in Soroban smart contracts is not recommended due to the predictability of these values. Block attributes are publicly visible and deterministic, making it easy for malicious actors to anticipate their values and manipulate outcomes to their advantage. Furthermore, validators could potentially influence these attributes, further exacerbating the risk of manipulation. For truly random number generation, it's important to use a source that is both unpredictable and external to the blockchain environment, reducing the potential for malicious exploitation."),(0,a.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,a.kt)("p",null,"Consider the following ",(0,a.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"\npub fn generate_random_value_timestamp(env: Env, max_val: u64) -> Result<u64, Error> {\n        if max_val == 0 {\n            Err(Error::MaxValZero)\n        } else {\n            let val = env.ledger().timestamp() % max_val;\n            Ok(val)\n        }\n    }\n    \n    pub fn generate_random_value_sequence(env: Env, max_val: u32) -> Result<u32, Error> {\n        if max_val == 0 {\n            Err(Error::MaxValZero)\n        } else {\n            let val = env.ledger().sequence() % max_val;\n            Ok(val)\n        }\n    }\n    \n")),(0,a.kt)("p",null,"The vulnerability lies in these functions use of blockchain-provided data like block timestamp and sequence number for pseudo-random number generation. This reliance on predictable blockchain data makes the generated values susceptible to manipulation by attackers."),(0,a.kt)("p",null,"The vulnerable code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1/vulnerable-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"remediation"},"Remediation"),(0,a.kt)("p",null,"Avoid using block attributes like ",(0,a.kt)("inlineCode",{parentName:"p"},"timestamp")," or ",(0,a.kt)("inlineCode",{parentName:"p"},"sequence")," for randomness generation, and consider using PRNG instead."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"\n   pub fn generate_random_value(env: Env, max_val: u64) -> Result<u64, Error> {\n        if max_val == 0 {\n            Err(Error::MaxValZero)\n        } else {\n            let val = env.prng().gen_range(0..max_val);\n            Ok(val)\n        }\n    }\n\n")),(0,a.kt)("p",null,"The remediated code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/blob/main/test-cases/insufficiently-random-values/insufficiently-random-values-1/remediated-example/src/lib.rs"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"references"},"References"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://dasp.co/#item-6"},"https://dasp.co/#item-6")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://blog.sigmaprime.io/solidity-security.html#SP-6"},"https://blog.sigmaprime.io/solidity-security.html#SP-6")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://swcregistry.io/docs/SWC-120"},"SWC-120")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://swcregistry.io/docs/SWC-116"},"SWC-116")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://ethernaut.openzeppelin.com/level/0x4dF32584890A0026e56f7535d0f2C6486753624f"},"Ethernaut: Coinflip")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#weak-PRNG"},"Slither: Weak PRNG")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#block-timestamp"},"Slither: Dangerous usage of block.timestamp"))))}m.isMDXComponent=!0}}]);