<h1 align="center">
    meteor :comet:
    <br>
</h1>

<h4 align="center">
Library for virtualization of the Rust language.<br>
Basic building block for meta-staging in Rust using procedural macros to write DSLs.
</h4>

#### :bell: WARNING :bell:
The library is an very early state, basically rendering it unusable. A lot of issues are still unsolved and the overall design will undergo drastic changes!

### Motivation

The project is motivated by the [`Scala LMS`](https://scala-lms.github.io//index.html) project and follows its virtualization approach. Implementing the virtualization process purely as library without compiler support would be very appealing for end-users. 'Virtualization' denotes the process of representing a parsed representation of Rust code in an overridable representation. 
