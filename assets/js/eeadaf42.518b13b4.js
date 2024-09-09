"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[2006],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>m});var r=n(9496);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var c=r.createContext({}),l=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},u=function(e){var t=l(e.components);return r.createElement(c.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},h=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,u=i(e,["components","mdxType","originalType","parentName"]),p=l(n),h=o,m=p["".concat(c,".").concat(h)]||p[h]||d[h]||a;return n?r.createElement(m,s(s({ref:t},u),{},{components:n})):r.createElement(m,s({ref:t},u))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,s=new Array(a);s[0]=h;var i={};for(var c in t)hasOwnProperty.call(t,c)&&(i[c]=t[c]);i.originalType=e,i[p]="string"==typeof e?e:o,s[1]=i;for(var l=2;l<a;l++)s[l]=n[l];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}h.displayName="MDXCreateElement"},6516:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>d,frontMatter:()=>a,metadata:()=>i,toc:()=>l});var r=n(2564),o=(n(9496),n(9613));const a={},s="Storage change events",i={unversionedId:"detectors/storage-change-events",id:"detectors/storage-change-events",title:"Storage change events",description:"Description",source:"@site/docs/detectors/24-storage-change-events.md",sourceDirName:"detectors",slug:"/detectors/storage-change-events",permalink:"/scout-soroban/docs/detectors/storage-change-events",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/24-storage-change-events.md",tags:[],version:"current",sidebarPosition:24,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Integer overflow or underflow",permalink:"/scout-soroban/docs/detectors/integer-overflow -or-underflow"},next:{title:"Token interface events",permalink:"/scout-soroban/docs/detectors/token-interface-events"}},c={},l=[{value:"Description",id:"description",level:2},{value:"Why is this bad?",id:"why-is-this-bad",level:2},{value:"Issue example",id:"issue-example",level:2},{value:"Remediated example",id:"remediated-example",level:2},{value:"How is it detected?",id:"how-is-it-detected",level:2}],u={toc:l},p="wrapper";function d(e){let{components:t,...n}=e;return(0,o.kt)(p,(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"storage-change-events"},"Storage change events"),(0,o.kt)("h2",{id:"description"},"Description"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"Category: ",(0,o.kt)("inlineCode",{parentName:"li"},"Best practices")),(0,o.kt)("li",{parentName:"ul"},"Severity: ",(0,o.kt)("inlineCode",{parentName:"li"},"Minor")),(0,o.kt)("li",{parentName:"ul"},"Detectors: ",(0,o.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/storage-change-events"},(0,o.kt)("inlineCode",{parentName:"a"},"storage-change-events"))),(0,o.kt)("li",{parentName:"ul"},"Test Cases: ",(0,o.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/storage-change-events/storage-change-events-1"},(0,o.kt)("inlineCode",{parentName:"a"},"storage-change-events-1")))),(0,o.kt)("p",null,"In Rust, it is very important to control storage, since it contains a large part of the information of a contract. For this reason, it is common to control storage movements through events, in order to record the changes that occur. If there is no control over these changes, it can lead to potential problems in the contract."),(0,o.kt)("h2",{id:"why-is-this-bad"},"Why is this bad?"),(0,o.kt)("p",null,"If there is no control over storage changes, it can lead to security and transparency issues within the contract."),(0,o.kt)("h2",{id:"issue-example"},"Issue example"),(0,o.kt)("p",null,"Consider the following ",(0,o.kt)("inlineCode",{parentName:"p"},"Soroban")," contract:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"\n  fn set_counter(env: Env, counter: CounterState) {\n        env.storage().instance().set(&STATE, &counter);\n    }\n\n")),(0,o.kt)("p",null,"In this example, the ",(0,o.kt)("inlineCode",{parentName:"p"},"set_counter()")," function does not emit an event to notify of a change in the storage."),(0,o.kt)("p",null,"The code example can be found ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/storage-change-events/storage-change-events-1/vulnerable-example"},"here"),"."),(0,o.kt)("h2",{id:"remediated-example"},"Remediated example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'    fn set_counter(env: Env, counter: CounterState) {\n        env.storage().instance().set(&STATE, &counter);\n        env.events()\n            .publish((COUNTER, symbol_short!("set")), counter.count);\n    }\n')),(0,o.kt)("p",null,"In this example, the ",(0,o.kt)("inlineCode",{parentName:"p"},"set_counter()")," function emits an event to notify of a change in the storage."),(0,o.kt)("p",null,"The remediated code example can be found ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/storage-change-events/storage-change-events-1/remediated-example"},"here"),"."),(0,o.kt)("h2",{id:"how-is-it-detected"},"How is it detected?"),(0,o.kt)("p",null,"Checks if the function emits an event in case a change has occurred in the storage."))}d.isMDXComponent=!0}}]);