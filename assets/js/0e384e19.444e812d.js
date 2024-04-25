"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[9671],{9613:(e,t,n)=>{n.d(t,{Zo:()=>d,kt:()=>g});var a=n(9496);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,a,o=function(e,t){if(null==e)return{};var n,a,o={},r=Object.keys(e);for(a=0;a<r.length;a++)n=r[a],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(a=0;a<r.length;a++)n=r[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var s=a.createContext({}),u=function(e){var t=a.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},d=function(e){var t=u(e.components);return a.createElement(s.Provider,{value:t},e.children)},c="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,o=e.mdxType,r=e.originalType,s=e.parentName,d=i(e,["components","mdxType","originalType","parentName"]),c=u(n),m=o,g=c["".concat(s,".").concat(m)]||c[m]||p[m]||r;return n?a.createElement(g,l(l({ref:t},d),{},{components:n})):a.createElement(g,l({ref:t},d))}));function g(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var r=n.length,l=new Array(r);l[0]=m;var i={};for(var s in t)hasOwnProperty.call(t,s)&&(i[s]=t[s]);i.originalType=e,i[c]="string"==typeof e?e:o,l[1]=i;for(var u=2;u<r;u++)l[u]=n[u];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},4771:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>l,default:()=>p,frontMatter:()=>r,metadata:()=>i,toc:()=>u});var a=n(2564),o=(n(9496),n(9613));const r={sidebar_position:1},l="Getting Started",i={unversionedId:"intro",id:"intro",title:"Getting Started",description:"Let's discover Scout in less than 5 minutes!.",source:"@site/docs/intro.md",sourceDirName:".",slug:"/intro",permalink:"/scout-soroban/docs/intro",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/intro.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"docsSidebar",next:{title:"Vulnerabilities",permalink:"/scout-soroban/docs/vulnerabilities/"}},s={},u=[{value:"About Scout",id:"about-scout",level:2},{value:"Features",id:"features",level:2},{value:"What you&#39;ll need",id:"what-youll-need",level:3},{value:"Command Line Interface (CLI)",id:"command-line-interface-cli",level:2},{value:"Installation",id:"installation",level:3},{value:"Usage",id:"usage",level:3},{value:"Profile configuration",id:"profile-configuration",level:2},{value:"HTML Vulnerability Report",id:"html-vulnerability-report",level:2},{value:"VSCode Extension",id:"vscode-extension",level:2},{value:"Installation",id:"installation-1",level:3},{value:"Usage",id:"usage-1",level:3},{value:"Troubleshooting Guide",id:"troubleshooting-guide",level:2},{value:"1. Installation Troubleshooting",id:"1-installation-troubleshooting",level:3},{value:"2. Crossed Contract Calls",id:"2-crossed-contract-calls",level:3}],d={toc:u},c="wrapper";function p(e){let{components:t,...r}=e;return(0,o.kt)(c,(0,a.Z)({},d,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"getting-started"},"Getting Started"),(0,o.kt)("p",null,"Let's discover ",(0,o.kt)("strong",{parentName:"p"},"Scout in less than 5 minutes!"),"."),(0,o.kt)("h2",{id:"about-scout"},"About Scout"),(0,o.kt)("p",null,"Scout is an extensible open-source tool intended to assist Stellar's Soroban smart contract developers and auditors detect common security issues and deviations from best practices. This tool helps developers write secure and more robust smart contracts."),(0,o.kt)("h2",{id:"features"},"Features"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"A list of vulnerabilities, best practices and enhancements, together with associated detectors to identify these issues in your code"),(0,o.kt)("li",{parentName:"ul"},"Command Line Interface (CLI)"),(0,o.kt)("li",{parentName:"ul"},"VSCode Extension")),(0,o.kt)("h3",{id:"what-youll-need"},"What you'll need"),(0,o.kt)("p",null,"Make sure that ",(0,o.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/cargo/getting-started/installation.html"},"Cargo")," is installed on your computer. For using the VSCode Extension you must be using ",(0,o.kt)("a",{parentName:"p",href:"https://code.visualstudio.com/"},"VSCode"),"."),(0,o.kt)("p",null,"You should be able to install and run Scout without issues on Mac or Linux. You can also use it in Windows through WSL."),(0,o.kt)("h2",{id:"command-line-interface-cli"},"Command Line Interface (CLI)"),(0,o.kt)("p",null,"The command line interface is designed to allow you to run Scout on an entire project. It is especially useful for auditing or performing a final review of your code."),(0,o.kt)("h3",{id:"installation"},"Installation"),(0,o.kt)("p",null,"FIn order to install the Command Line Interface, first install Scout dependencies by running the following command:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"cargo install cargo-dylint dylint-link\n")),(0,o.kt)("p",null,"Afterwards, install Scout with the following command:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"cargo install cargo-scout-audit\n")),(0,o.kt)("h3",{id:"usage"},"Usage"),(0,o.kt)("p",null,"To run Scout on your project, navigate to its root directory and execute the following command:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"cargo scout-audit\n")),(0,o.kt)("p",null,"In the table below, we specify all the option available for the CLI."),(0,o.kt)("table",null,(0,o.kt)("thead",{parentName:"table"},(0,o.kt)("tr",{parentName:"thead"},(0,o.kt)("th",{parentName:"tr",align:null},"Command/Option"),(0,o.kt)("th",{parentName:"tr",align:null},"Explanation"))),(0,o.kt)("tbody",{parentName:"table"},(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit")),(0,o.kt)("td",{parentName:"tr",align:null},"Runs the static analyzer on the current directory")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --help")),(0,o.kt)("td",{parentName:"tr",align:null},"Provides a brief explanation of all the available commands and their usage.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --manifest-path <PATH_TO_CARGO_TOML>")),(0,o.kt)("td",{parentName:"tr",align:null},"This option is used to specify the path to the Cargo.toml file that you want to analyze.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --profile <PROFILE_NAME>")),(0,o.kt)("td",{parentName:"tr",align:null},"This option allows you to analyze code using specific group of detectors, configured previously on ",(0,o.kt)("inlineCode",{parentName:"td"},"$HOME/.config/scout/(ink/soroban)-config.toml"))),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --filter <DETECTOR_LIST_SEPARATED_BY_COMAS>")),(0,o.kt)("td",{parentName:"tr",align:null},"This option allows you to analyze code using specific detectors. Provide a comma-separated list of detectors for this purpose.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --exclude <DETECTOR_LIST_SEPARATED_BY_COMAS>")),(0,o.kt)("td",{parentName:"tr",align:null},"With this command, you can exclude specific detectors from the analysis. You need to give a comma-separated list of the detectors to be excluded.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --list-detectors")),(0,o.kt)("td",{parentName:"tr",align:null},"Display a list of all available detectors.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --version")),(0,o.kt)("td",{parentName:"tr",align:null},"Displays the current version of the static analyzer.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --verbose")),(0,o.kt)("td",{parentName:"tr",align:null},"Print additional information on run")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --local-detectors <PATH_TO_FOLDER>")),(0,o.kt)("td",{parentName:"tr",align:null},"Uses the detectors of a local folder. This considers the sub-folders as detectors.")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --output-format [text,json,html,sarif,pdf,md,markdown]")),(0,o.kt)("td",{parentName:"tr",align:null},"Sets the output format. Selecting ",(0,o.kt)("inlineCode",{parentName:"td"},"json"),", ",(0,o.kt)("inlineCode",{parentName:"td"},"html"),", ",(0,o.kt)("inlineCode",{parentName:"td"},"sarif"),", ",(0,o.kt)("inlineCode",{parentName:"td"},"markdown"),", or ",(0,o.kt)("inlineCode",{parentName:"td"},"pdf")," will create a file with the output")),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("inlineCode",{parentName:"td"},"cargo scout-audit --output-path <PATH_TO_OUTPUT_FILE>")),(0,o.kt)("td",{parentName:"tr",align:null},"Sets the output path. If a format was selected, this will replace the default file with the given one")))),(0,o.kt)("h2",{id:"profile-configuration"},"Profile configuration"),(0,o.kt)("p",null,"The profile configuration file is generated automatically in ",(0,o.kt)("inlineCode",{parentName:"p"},"$HOME/.config/scout/soroban-config.toml")," the first time scout-audit is run.\nThe configuration has the following format"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-toml"},"[<profile-name>.<detector-name>]\nenabled = <true|false>\n")),(0,o.kt)("p",null,"For example, if you want to define a profile named 'dev' in which the 'panic-error' detector is disabled and the 'soroban-version' detector is enabled, you should do the following:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-toml"},"[dev.panic-error]\nenabled = false\n[dev.soroban-version]\nenabled = true\n")),(0,o.kt)("h2",{id:"html-vulnerability-report"},"HTML Vulnerability Report"),(0,o.kt)("p",null,"We've upgraded Scout's HTML output to introduce a comprehensive HTML Vulnerability Report, enhancing your ability to quickly assess and address the security status of your project. The new features included in the report are designed to provide a detailed yet concise overview of the findings."),(0,o.kt)("p",null,(0,o.kt)("img",{alt:"html",src:n(1370).Z,width:"1895",height:"893"})),(0,o.kt)("p",null,"Usage: ",(0,o.kt)("inlineCode",{parentName:"p"},"cargo scout-audit --output-format html")),(0,o.kt)("h2",{id:"vscode-extension"},"VSCode Extension"),(0,o.kt)("p",null,"We built the Scout VSCode Extension to help developers write secure and more robust smart contracts. Listing security issues, and highlighting issues with squiggles and hover-over descriptions, we hope our extension will help you catch vulnerabilities during development."),(0,o.kt)("h3",{id:"installation-1"},"Installation"),(0,o.kt)("p",null,"Install Scout from the Marketplace within the Extensions tab of Visual Studio Code. You can find the extension ",(0,o.kt)("a",{parentName:"p",href:"https://marketplace.visualstudio.com/items?itemName=CoinFabrik.scout-audit"},"here"),"."),(0,o.kt)("p",null,"You'll also need to have installed the CLI, as the extension uses the CLI to perform the analysis. You can find instructions for installing the CLI ",(0,o.kt)("a",{parentName:"p",href:"#command-line-interface-cli"},"here"),"."),(0,o.kt)("h3",{id:"usage-1"},"Usage"),(0,o.kt)("p",null,"After you've installed the extension, simply open a project workspace that contains any Soroban (.rs) files. You will see potential issues and warnings via a wiggle underline of the relevant code."),(0,o.kt)("h2",{id:"troubleshooting-guide"},"Troubleshooting Guide"),(0,o.kt)("h3",{id:"1-installation-troubleshooting"},"1. Installation Troubleshooting"),(0,o.kt)("p",null,(0,o.kt)("strong",{parentName:"p"},"Issue"),": Difficulties installing dependencies and Scout."),(0,o.kt)("p",null,(0,o.kt)("strong",{parentName:"p"},"Solution"),":"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"For Dylint:")),(0,o.kt)("p",null,"To install necessary libraries for Dylint, run the following commands:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"sudo apt install libssl-dev\nsudo apt install pkg-config\n")),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"For C Compiler (gcc). ")),(0,o.kt)("p",null,"To install gcc, which is required for some components, use:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"sudo apt install gcc\n")),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"For error ",(0,o.kt)("inlineCode",{parentName:"li"},"error[E0658]"),". ")),(0,o.kt)("p",null,"When encountering this error ",(0,o.kt)("inlineCode",{parentName:"p"},"error[E0658]: use of unstable library feature 'stdsimd'"),", run cargo clean and ensure you are using this version of rustup:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"}," cargo clean\n rustup default nightly-2023-12-16\n")),(0,o.kt)("h3",{id:"2-crossed-contract-calls"},"2. Crossed Contract Calls"),(0,o.kt)("p",null,(0,o.kt)("strong",{parentName:"p"},"Issue"),": Scout encounters issues when analyzing contracts that perform crossed calls."),(0,o.kt)("p",null,(0,o.kt)("strong",{parentName:"p"},"Solution"),":"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"When encountering problems with crossed calls, it's beneficial to compile the dependent contract first. Run the following command to build the second contract:")),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"  soroban contract build\n")))}p.isMDXComponent=!0},1370:(e,t,n)=>{n.d(t,{Z:()=>a});const a=n.p+"assets/images/html-ec01cddcb4a4f6d587bdf23460fde7b9.png"}}]);