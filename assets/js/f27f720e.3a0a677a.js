"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[4162],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>m});var n=r(9496);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function s(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function a(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):s(s({},t),e)),r},u=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,i=e.originalType,c=e.parentName,u=a(e,["components","mdxType","originalType","parentName"]),p=l(r),f=o,m=p["".concat(c,".").concat(f)]||p[f]||d[f]||i;return r?n.createElement(m,s(s({ref:t},u),{},{components:r})):n.createElement(m,s({ref:t},u))}));function m(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=r.length,s=new Array(i);s[0]=f;var a={};for(var c in t)hasOwnProperty.call(t,c)&&(a[c]=t[c]);a.originalType=e,a[p]="string"==typeof e?e:o,s[1]=a;for(var l=2;l<i;l++)s[l]=r[l];return n.createElement.apply(null,s)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},726:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>d,frontMatter:()=>i,metadata:()=>a,toc:()=>l});var n=r(2564),o=(r(9496),r(9613));const i={sidebar_position:3},s="Detectors",a={unversionedId:"detectors/README",id:"detectors/README",title:"Detectors",description:"In this section we introduce our set of detectors powered by Dylint - a Rust linting tool.",source:"@site/docs/detectors/README.md",sourceDirName:"detectors",slug:"/detectors/",permalink:"/scout-soroban/docs/detectors/",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/detectors/README.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"docsSidebar",previous:{title:"Soroban version",permalink:"/scout-soroban/docs/vulnerabilities/soroban-version"},next:{title:"Divide before multiply",permalink:"/scout-soroban/docs/detectors/divide-before-multiply"}},c={},l=[],u={toc:l},p="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(p,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"detectors"},"Detectors"),(0,o.kt)("p",null,"In this section we introduce our set of detectors powered by ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/trailofbits/dylint"},"Dylint")," - a Rust linting tool. "),(0,o.kt)("p",null,"Similar to ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/rust-lang/rust-clippy"},"Clippy"),", Dylint can run lints to help identify potential issues in code. However, unlike Clippy, Dylint can run lints from user-specified dynamic libraries instead of just a statically predetermined set. This unique feature of Dylint makes it easier for developers to extend and customize their own personal lint collections, leading to reduced compile and run cycles."),(0,o.kt)("p",null,"Check our ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/web3-grant/tree/main/detectors"},"Proof of Concept Study")," for a more detailed analysis of different detection techniques and tools."))}d.isMDXComponent=!0}}]);