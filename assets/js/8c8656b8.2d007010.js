"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[5083],{9613:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>b});var a=n(9496);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var l=a.createContext({}),p=function(e){var t=a.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},c=function(e){var t=p(e.components);return a.createElement(l.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,l=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),u=p(n),m=r,b=u["".concat(l,".").concat(m)]||u[m]||d[m]||i;return n?a.createElement(b,o(o({ref:t},c),{},{components:n})):a.createElement(b,o({ref:t},c))}));function b(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,o=new Array(i);o[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[u]="string"==typeof e?e:r,o[1]=s;for(var p=2;p<i;p++)o[p]=n[p];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},3850:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>s,toc:()=>p});var a=n(2564),r=(n(9496),n(9613));const i={},o="Unprotected mapping operation",s={unversionedId:"vulnerabilities/unprotected-mapping-operation",id:"vulnerabilities/unprotected-mapping-operation",title:"Unprotected mapping operation",description:"Description",source:"@site/docs/vulnerabilities/16-unprotected-mapping-operation.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/unprotected-mapping-operation",permalink:"/scout-soroban/docs/vulnerabilities/unprotected-mapping-operation",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/vulnerabilities/16-unprotected-mapping-operation.md",tags:[],version:"current",sidebarPosition:16,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Assert violation",permalink:"/scout-soroban/docs/vulnerabilities/assert-violation"},next:{title:"Unrestricted Transfer From",permalink:"/scout-soroban/docs/vulnerabilities/unrestricted-transfer-from"}},l={},p=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2}],c={toc:p},u="wrapper";function d(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"unprotected-mapping-operation"},"Unprotected mapping operation"),(0,r.kt)("h2",{id:"description"},"Description"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,r.kt)("inlineCode",{parentName:"li"},"Authorization")),(0,r.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,r.kt)("inlineCode",{parentName:"li"},"Critical")),(0,r.kt)("li",{parentName:"ul"},"Detectors: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unprotected-mapping-operation"},(0,r.kt)("inlineCode",{parentName:"a"},"unprotected-mapping-operation"))),(0,r.kt)("li",{parentName:"ul"},"Test Cases: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation-1"},(0,r.kt)("inlineCode",{parentName:"a"},"unprotected-mapping-operation-1")))),(0,r.kt)("p",null,"Modifying mappings with an arbitrary key given by users can be a significant vulnerability for several reasons:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Unintended Modifications: Allowing users to provide arbitrary keys can lead to unintended modifications of critical data within the smart contract. If the input validation and sanitation are not done properly, users may be able to manipulate the data in ways that were not intended by the contract's author.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Data Corruption: Malicious users could intentionally provide keys that result in the corruption or manipulation of important data stored in the mapping. This could lead to incorrect calculations, unauthorized access, or other undesirable outcomes.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},"Denial-of-Service (DoS) Attacks: If users can set arbitrary keys, they may be able to create mappings with a large number of entries, potentially causing the contract to exceed its gas limit. This could lead to denial-of-service attacks, making the contract unusable for other users."))),(0,r.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,r.kt)("p",null,"Consider the following ",(0,r.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"   pub fn set_balance(env: Env, address: Address, balance: i128) -> State {\n        // Get the current state.\n        let mut state = Self::get_state(env.clone());\n\n        // Set the new account to have total supply if it doesn't exist.\n        if !state.balances.contains_key(address.clone()) {\n            state.balances.set(address, balance);\n            // Save the state.\n            env.storage().persistent().set(&STATE, &state);\n        }\n\n        state\n    }\n")),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"set_balance()")," function allows anyone to call it and modify the account balances in the state. It lacks authorization checks and allows modifying the mutable state directly."),(0,r.kt)("p",null,"The vulnerable code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation1/vulnerable-example"},(0,r.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,r.kt)("h2",{id:"remediation"},"Remediation"),(0,r.kt)("p",null,"The fix adds an ",(0,r.kt)("inlineCode",{parentName:"p"},"address.require_auth()")," step, likely checking user permissions to update balances. This ensures only authorized users can modify account data."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"    pub fn set_balance(env: Env, address: Address, balance: i128) -> State {\n        // Authenticate user\n        address.require_auth();\n\n        // Get the current state.\n        let mut state = Self::get_state(env.clone());\n\n        // Set the new account to have total supply if it doesn't exist.\n        if !state.balances.contains_key(address.clone()) {\n            state.balances.set(address, balance);\n            // Save the state.\n            env.storage().persistent().set(&STATE, &state);\n        }\n\n        state\n    }\n")),(0,r.kt)("p",null,"The remediated code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation1/remediated-example"},(0,r.kt)("inlineCode",{parentName:"a"},"here")),"."))}d.isMDXComponent=!0}}]);