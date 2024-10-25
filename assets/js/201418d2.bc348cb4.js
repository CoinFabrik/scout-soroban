"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[1867],{9613:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>f});var a=n(9496);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var c=a.createContext({}),l=function(e){var t=a.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=l(e.components);return a.createElement(c.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),u=l(n),d=r,f=u["".concat(c,".").concat(d)]||u[d]||m[d]||o;return n?a.createElement(f,i(i({ref:t},p),{},{components:n})):a.createElement(f,i({ref:t},p))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=d;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s[u]="string"==typeof e?e:r,i[1]=s;for(var l=2;l<o;l++)i[l]=n[l];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},3845:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>i,default:()=>m,frontMatter:()=>o,metadata:()=>s,toc:()=>l});var a=n(2564),r=(n(9496),n(9613));const o={},i="Token interface events",s={unversionedId:"detectors/token-interface-events",id:"detectors/token-interface-events",title:"Token interface events",description:"Description",source:"@site/docs/detectors/24-token-interface-events.md",sourceDirName:"detectors",slug:"/detectors/token-interface-events",permalink:"/scout-soroban/docs/detectors/token-interface-events",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/24-token-interface-events.md",tags:[],version:"current",sidebarPosition:24,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Storage change events",permalink:"/scout-soroban/docs/detectors/storage-change-events"},next:{title:"Contribute",permalink:"/scout-soroban/docs/contribute"}},c={},l=[{value:"Description",id:"description",level:2},{value:"Why is this bad?",id:"why-is-this-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2}],p={toc:l},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"token-interface-events"},"Token interface events"),(0,r.kt)("h2",{id:"description"},"Description"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Category: ",(0,r.kt)("inlineCode",{parentName:"li"},"Best practices")),(0,r.kt)("li",{parentName:"ul"},"Severity: ",(0,r.kt)("inlineCode",{parentName:"li"},"Medium")),(0,r.kt)("li",{parentName:"ul"},"Detectors: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/token-interface-events"},(0,r.kt)("inlineCode",{parentName:"a"},"token-interface-events"))),(0,r.kt)("li",{parentName:"ul"},"Test Cases: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/token-interface-events/token-interface-events-1"},(0,r.kt)("inlineCode",{parentName:"a"},"token-interface-events-1")))),(0,r.kt)("p",null,"In Rust, the token contracts have a special interface with certain requirements. One of these requirements is related to events; this requirement states that token functions must emit the events in the specified format. If this does not happen, the contract will have potential errors."),(0,r.kt)("h2",{id:"why-is-this-bad"},"Why is this bad?"),(0,r.kt)("p",null,"If the token functions do not emit events, the following errors may occur:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Token standard compliance")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Transparency: Events provide a transparent way to log and broadcast important actions like token transfers, approvals, and minting/burning. This transparency is crucial for users, developers, and external systems to monitor and react to contract activities.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Interoperability: Many decentralized applications (dApps) rely on events to interact with tokens. Without events, these applications might not be able to function correctly, as they would have no way of knowing when a transfer or other important action has occurred. Also, off-chain systems, like wallets, exchanges, and block explorers, use events to track token activity. If events are not implemented, these systems may encounter errors in providing accurate and real-time information about the token.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Debugging and Auditing: Events are very helpful for debugging and auditing smart contracts. They are useful because they provide detailed information about what happened in the contract during execution."))),(0,r.kt)("h2",{id:"issue-example"},"Issue example"),(0,r.kt)("p",null,"Consider the following ",(0,r.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"\n  fn transfer(env: Env, from: Address, to: Address, amount: i128) {\n        from.require_auth();\n        let from_balance = Self::balance(env.clone(), from.clone());\n        let to_balance = Self::balance(env.clone(), to.clone());\n        assert!(from_balance >= amount);\n        env.storage()\n            .instance()\n            .set(&DataKey::Balance(from), &(from_balance - amount));\n        env.storage()\n            .instance()\n            .set(&DataKey::Balance(to), &(to_balance + amount));\n    }\n\n")),(0,r.kt)("p",null,"In this example, the ",(0,r.kt)("inlineCode",{parentName:"p"},"transfer()")," function does not emit an event."),(0,r.kt)("p",null,"The code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/token-interface-events/token-interface-events-1/vulnerable-example"},"here"),"."),(0,r.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"  fn transfer(env: Env, from: Address, to: Address, amount: i128) {\n        from.require_auth();\n        let from_balance = Self::balance(env.clone(), from.clone());\n        let to_balance = Self::balance(env.clone(), to.clone());\n        assert!(from_balance >= amount);\n        env.storage()\n            .instance()\n            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));\n        env.storage()\n            .instance()\n            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));\n\n        TokenUtils::new(&env).events().transfer(from, to, amount);\n    }\n")),(0,r.kt)("p",null,"In this example, the ",(0,r.kt)("inlineCode",{parentName:"p"},"transfer()")," function emits an event."),(0,r.kt)("p",null,"The remediated code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/token-interface-events/token-interface-events-1/remediated-example"},"here"),"."),(0,r.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,r.kt)("p",null,"If the token interface trait is being used, check if all of the token's functions emit events."))}m.isMDXComponent=!0}}]);