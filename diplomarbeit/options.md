
# Options for using the diploma thesis template with the htldoc tool

### title
```
title = "TITLE";
```

### subtitle
```
subtitle = "SUBTITLE";
```

### institute
```
institute = "ABTEILUNG";
```
What we in german call "Die Abteilung", that you are a part of.

### submissionDate
```
submissionDate = "2025-12-24";
```
Either in format yyyy-mm-dd or in dd.mm.yyyy.

### authors
```
authors = [ "AUTHOR 1" "AUTHOR 2" ];
```

### supervisors
```
supervisors = [ "SUPERVISOR 1" "SUPERVISOR 2" ];
```

### partner
```
partner = "....";
```
The company you make the diploma thesis for, if you do so.

### subject
```
subject = [ "HWE" ];
```

### keywords
```
keywords = [ "KEYWORD 1" "KEYWORD 2" ];
```

### lang
```
lang = "ngerman";
```
The language of the document. For german use `ngerman` and for english `american`. The default value is ngerman.

### twoSidePrinting
```
twoSidePrinting = true;
```
Enables this weired layout, that makes some pages more to the right and some more to the left. This is for when the document is to be printed in a twoside manner, but maybe should be disabled when making a pdf for digital use.

TODO: probably also with one side, it's to the side????


### draftMode
```
draftMode = false;
```
If true, included graphics are replaced by empty rectangles (of same size) and overfull boxes (in margin space) are marked with black box (-> easy to spot!).


### htldocBuildDir
```
htldocVersion = "github:c2vi/htldoc/master";
```
The Nix Flake URL of the htldoc Version to use for the build.



