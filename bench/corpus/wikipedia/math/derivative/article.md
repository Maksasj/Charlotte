# Derivative

In mathematics, the derivative is a fundamental tool that quantifies the sensitivity to change of a function's output with respect to its input. The derivative of a function of a single variable at a chosen input value, when it exists, is the slope of the tangent line to the graph of the function at that point. The tangent line is the best linear approximation of the function near that input value. The derivative is often described as the instantaneous rate of change, the ratio of the instantaneous change in the dependent variable to that of the independent variable. The process of finding a derivative is called differentiation.

There are multiple different notations for differentiation. Leibniz notation, named after Gottfried Wilhelm Leibniz, is represented as the ratio of two differentials, whereas prime notation is written by adding a prime mark. Higher order notations represent repeated differentiation, and they are usually denoted in Leibniz notation by adding superscripts to the differentials, and in prime notation by adding additional prime marks. Higher order derivatives are used in physics; for example, the first derivative  with respect to time of the position of a moving object is its velocity, and the second derivative is its acceleration.

Derivatives can be generalized to functions of several real variables. In this case, the derivative is reinterpreted as a linear transformation whose graph is (after an appropriate translation) the best linear approximation to the graph of the original function. The Jacobian matrix is the matrix that represents this linear transformation with respect to the basis given by the choice of independent and dependent variables.  It can be calculated in terms of the partial derivatives with respect to the independent variables.  For a real-valued function of several variables, the Jacobian matrix reduces to the gradient vector.

### As a limit

A function of a real variable f(x) is differentiable at a point a of its domain, if its domain contains an open interval containing ⁠a⁠, and the limit

L=limh→0f(a+h)−f(a)h

exists.  This means that, for every positive real number ⁠ε⁠, there exists a positive real number δ such that, for every h such that |h|<δ and h≠0 then f(a+h) is defined, and

|L−f(a+h)−f(a)h|<ε,

where the vertical bars denote the absolute value. This is an example of the (ε, δ)-definition of limit.

If the function f is differentiable at ⁠a⁠, that is if the limit L exists, then this limit is called the derivative of f at ⁠a⁠. Multiple notations for the derivative exist. The derivative of f at a can be denoted ⁠f′(a)⁠, read as "⁠f⁠ prime of ⁠a⁠"; or it can be denoted ⁠dfdx(a)⁠, read as "the derivative of f with respect to x at ⁠a⁠" or "⁠df⁠ by (or over) dx at ⁠a⁠". See § Notation below. If f is a function that has a derivative at every point in its domain, then a function can be defined by mapping every point x to the value of the derivative of f at ⁠x⁠. This function is written f′ and is called the derivative function or the derivative of ⁠f⁠. The function f sometimes has a derivative at most, but not all, points of its domain. The function whose value at a equals f′(a) whenever f′(a) is defined and elsewhere is undefined is also called the derivative of ⁠f⁠. It is still a function, but its domain may be smaller than the domain of ⁠f⁠.

For example, let f be the squaring function: ⁠f(x)=x2⁠. Then the quotient in the definition of the derivative is

f(a+h)−f(a)h=(a+h)2−a2h=a2+2ah+h2−a2h=2a+h.

The division in the last step is valid as long as ⁠h≠0⁠. The closer h is to ⁠0⁠, the closer this expression becomes to the value ⁠2a⁠. The limit exists, and for every input a the limit is ⁠2a⁠. So, the derivative of the squaring function is the doubling function: ⁠f′(x)=2x⁠.

The ratio in the definition of the derivative is the slope of the line through two points on the graph of the function ⁠f⁠, specifically the points (a,f(a)) and ⁠(a+h,f(a+h))⁠. As h is made smaller, these points grow closer together, and the slope of this line approaches the limiting value, the slope of the tangent to the graph of f at ⁠a⁠. In other words, the derivative is the slope of the tangent.

### Using infinitesimals

One way to think of the derivative dfdx(a) is as the ratio of an infinitesimal change in the output of the function f to an infinitesimal change in its input. In order to make this intuition rigorous, a system of rules for manipulating infinitesimal quantities is required. The system of hyperreal numbers is a way of treating infinite and infinitesimal quantities.  The hyperreals are an extension of the real numbers that contain numbers greater than anything of the form 1+1+⋯+1 for any finite number of terms. Such numbers are infinite, and their reciprocals are infinitesimals. The application of hyperreal numbers to the foundations of calculus is called nonstandard analysis. This provides a way to define the basic concepts of calculus such as the derivative and integral in terms of infinitesimals, thereby giving a precise meaning to the d in the Leibniz notation. Thus, the derivative of f(x) becomes f′(x)=st⁡(f(x+dx)−f(x)dx) for an arbitrary infinitesimal ⁠dx⁠, where st denotes the standard part function, which "rounds off" each finite hyperreal to the nearest real. Taking the squaring function f(x)=x2 as an example again,

f′(x)=st⁡(x2+2x⋅dx+(dx)2−x2dx)=st⁡(2x⋅dx+(dx)2dx)=st⁡(2x⋅dxdx+(dx)2dx)=st⁡(2x+dx)=2x.

## Continuity and differentiability

If f is differentiable at ⁠a⁠, then f must also be continuous at ⁠a⁠. As an example, choose a point a and let f be the step function that returns the value 1 for all x less than ⁠a⁠, and returns a different value 10 for all x greater than or equal to ⁠a⁠.  The function f cannot have a derivative at ⁠a⁠. If h is negative, then a+h is on the low part of the step, so the secant line from a to a+h is very steep; as h tends to zero, the slope tends to infinity. If h is positive, then a+h is on the high part of the step, so the secant line from a to a+h has slope zero. Consequently, the secant lines do not approach any single slope, so the limit of the difference quotient does not exist. However, even if a function is continuous at a point, it may not be differentiable there. For example, the absolute value function given by f(x)=|x| is continuous at ⁠x=0⁠, but it is not differentiable there. If h is positive, then the slope of the secant line from 0 to h is one; if h is negative, then the slope of the secant line from 0 to h is ⁠−1⁠.  This can be seen graphically as a "kink" or a "cusp" in the graph at ⁠x=0⁠. Even a function with a smooth graph is not differentiable at a point where its tangent is vertical: For instance, the function given by f(x)=x1/3 is not differentiable at ⁠x=0⁠. In summary, a function that has a derivative is continuous, but there are continuous functions that do not have a derivative.

Most functions that occur in practice have derivatives at all points or almost every point. Early in the history of calculus, many mathematicians assumed that a continuous function was differentiable at most points. Under mild conditions (for example, if the function is a monotone or a Lipschitz function), this is true. However, in 1872, Weierstrass found the first example of a function that is continuous everywhere but differentiable nowhere.  This example is now known as the Weierstrass function. In 1931, Stefan Banach proved that the set of functions that have a derivative at some point is a meager set in the space of all continuous functions. Informally, this means that hardly any random continuous functions have a derivative at even one point.

## Notation

One common way of writing the derivative of a function is Leibniz notation, introduced by Gottfried Wilhelm Leibniz in 1675, which denotes a derivative as the quotient of two differentials, such as dy and ⁠dx⁠. It is still commonly used when the equation y=f(x) is viewed as a functional relationship between dependent and independent variables. The first derivative is denoted by ⁠dydx⁠, read as "the derivative of y with respect to ⁠x⁠". This derivative can alternately be treated as the application of a differential operator to a function, dydx=ddxf(x). Higher derivatives are expressed using the notation dnydxn for the nth derivative of ⁠y=f(x)⁠. These are abbreviations for multiple applications of the derivative operator; for example, d2ydx2=ddx(ddxf(x)). Unlike some alternatives, Leibniz notation involves explicit specification of the variable for differentiation, in the denominator, which removes ambiguity when working with multiple interrelated quantities. The derivative of a composed function can be expressed using the chain rule: if u=g(x) and y=f(g(x)) then dydx=dydu⋅dudx.

Another common notation for differentiation is by using the prime mark in the symbol of a function ⁠f(x)⁠. This notation, due to Joseph-Louis Lagrange, is now known as prime notation. The first derivative is written as ⁠f′(x)⁠, read as "⁠f⁠ prime of ⁠x⁠", or ⁠y′⁠, read as "⁠y⁠ prime". Similarly, the second and the third derivatives can be written as f″ and ⁠f‴⁠, respectively. For denoting the number of higher derivatives beyond this point, some authors use Roman numerals in superscript, whereas others place the number in parentheses, such as fiv or ⁠f(4)⁠. The latter notation generalizes to yield the notation f(n) for the ⁠n⁠th derivative of ⁠f⁠.

In Newton's notation or the dot notation, a dot is placed over a symbol to represent a time derivative. If y is a function of ⁠t⁠, then the first and second derivatives can be written as y˙ and ⁠y¨⁠, respectively. This notation is used exclusively for derivatives with respect to time or arc length. It is typically used in differential equations in physics and differential geometry. However, the dot notation becomes unmanageable for high-order derivatives (of order 4 or more) and cannot deal with multiple independent variables.

Another notation is D-notation, which represents the differential operator by the symbol ⁠D⁠. The first derivative is written Df(x) and higher derivatives are written with a superscript, so the nth derivative is ⁠Dnf(x)⁠. This notation is sometimes called Euler notation, although it seems that Leonhard Euler did not use it, and the notation was introduced by Louis François Antoine Arbogast. To indicate a partial derivative, the variable differentiated by is indicated with a subscript, for example given the function ⁠u=f(x,y)⁠, its partial derivative with respect to x can be written Dxu or ⁠Dxf(x,y)⁠. Higher partial derivatives can be indicated by superscripts or multiple subscripts, e.g. Dxyf(x,y)=∂∂y(∂∂xf(x,y)) and ⁠Dx2f(x,y)=∂∂x(∂∂xf(x,y))⁠.

## Rules of computation

In principle, the derivative of a function can be computed from the definition by considering the difference quotient and computing its limit. Once the derivatives of a few simple functions are known, the derivatives of functions obtained from these simple functions by applying elementary operations, such as products, sums, quotients, or function composition, to them can be determined by applying rules for differentiation. This process of finding a derivative is called differentiation.

### Rules for basic functions

The following are the rules for the derivatives of the most common basic functions. Here, a is a real number, and e is the base of the natural logarithm, approximately 2.71828.

Derivatives of powers

ddxxa=axa−1

Functions of exponential, natural logarithm, and logarithm with general base

ddxex=ex

⁠ddxax=axln⁡(a)⁠, for a>0

⁠ddxln⁡(x)=1x⁠, for x>0

⁠ddxloga(x)=1xln⁡(a)⁠, for x,a>0

Trigonometric functions

ddxsin⁡(x)=cos⁡(x)

ddxcos⁡(x)=−sin⁡(x)

ddxtan⁡(x)=sec2(x)=1cos2(x)=1+tan2(x)

Inverse trigonometric functions

⁠ddxarcsin⁡(x)=11−x2⁠, for −1<x<1

⁠ddxarccos⁡(x)=−11−x2⁠, for −1<x<1

ddxarctan⁡(x)=11+x2

### Rules for combined functions

The following rules allow deducing derivatives of many functions from the derivatives of the basic functions:

Constant rule

If f is a constant function, then for all ⁠x⁠, f′(x)=0.

Sum rule

(αf+βg)′=αf′+βg′ for all functions f and g and all real numbers α and ⁠β⁠.

Product rule

(fg)′=f′g+fg′ for all functions f and ⁠g⁠. As a special case, this rule includes the fact (αf)′=αf′ whenever α is a constant because α′f=0⋅f=0 by the constant rule.

Quotient rule

(fg)′=f′g−fg′g2  for all functions f and g at all inputs where ⁠g≠0⁠.

Chain rule for composite functions

If ⁠f(x)=h(g(x))⁠, then f′(x)=h′(g(x))⋅g′(x).

### Computation example

The derivative of the function given by f(x)=x4+sin⁡(x2)−ln⁡(x)ex+7 is

f′(x)=4x(4−1)+d(x2)dxcos⁡(x2)−d(ln⁡x)dxex−ln⁡(x)d(ex)dx+0=4x3+2xcos⁡(x2)−1xex−ln⁡(x)ex.

Here the second term was computed using the chain rule and the third term using the product rule. The known derivatives of the elementary functions ⁠x2⁠, ⁠x4⁠, ⁠sin⁡(x)⁠, ⁠ln⁡(x)⁠, and ⁠exp⁡(x)⁠, as well as the constant ⁠7⁠, were also used.

## Antidifferentiation

An antiderivative of a function f is a function whose derivative is ⁠f⁠. Antiderivatives are not unique: if A is an antiderivative of ⁠f⁠, then so is ⁠A+c⁠, where c is any constant, because the derivative of a constant is zero. The fundamental theorem of calculus shows that finding an antiderivative of a function gives a way to compute the areas of shapes bounded by that function. More precisely, the integral of a function over a closed interval is equal to the difference between the values of an antiderivative evaluated at the endpoints of that interval.

## Higher-order derivatives

Higher-order derivatives are the result of differentiating a function repeatedly. Given that f is a differentiable function, the derivative of f is the first derivative, denoted as ⁠f′⁠. The derivative of f′ is the second derivative, denoted as ⁠f″⁠, and the derivative of f″ is the third derivative, denoted as ⁠f‴⁠. By continuing this process, if it exists, the ⁠n⁠th derivative is the derivative of the ⁠(n−1)⁠th derivative or the derivative of order ⁠n⁠. As has been discussed above, the generalization of derivative of a function f may be denoted as ⁠f(n)⁠.  A function that has k successive derivatives is called k times differentiable. If the k-th derivative is continuous, then the function is said to be of differentiability class ⁠Ck⁠. A function that has infinitely many derivatives is called infinitely differentiable or smooth. Any polynomial function is infinitely differentiable; taking derivatives repeatedly will eventually result in a constant function, and all subsequent derivatives of that function are zero.

One application of higher-order derivatives is in physics. For example, if the function ⁠x⁠ represents an object's position with respect to time, ⁠x′⁠ represents the object's velocity, ⁠x″⁠ represents the object's acceleration, and ⁠x‴⁠ represents the object's jerk.

### Vector-valued functions

A vector-valued function 𝐲 of a real variable sends real numbers to vectors in some vector space ⁠ℝn⁠. A vector-valued function can be split up into its coordinate functions ⁠y1(t),y2(t),…,yn(t)⁠, meaning that ⁠𝐲=(y1(t),y2(t),…,yn(t))⁠. This includes, for example, parametric curves in ℝ2 or ⁠ℝ3⁠. The coordinate functions are real-valued functions, so the above definition of derivative applies to them. The derivative of 𝐲(t) is defined to be the vector, called the tangent vector, whose coordinates are the derivatives of the coordinate functions. That is,

𝐲′(t)=limh→0𝐲(t+h)−𝐲(t)h,

if the limit exists. The subtraction in the numerator is the subtraction of vectors, not scalars. If the derivative of 𝐲 exists for every value of ⁠t⁠, then 𝐲′ is another vector-valued function.

### Partial derivatives

Functions can depend upon more than one variable. A partial derivative of a function of several variables is its derivative with respect to one of those variables, with the others held constant. Partial derivatives are used in vector calculus and differential geometry. As with ordinary derivatives, multiple notations exist: the partial derivative of a function f(x,y,…) with respect to the variable x is variously denoted by

among other possibilities. It can be thought of as the rate of change of the function in the x-direction. Here ∂ is a rounded d called the partial derivative symbol.  To distinguish it from the letter d, ∂ is sometimes pronounced "der", "del", or "partial" instead of "dee". For example, let ⁠f(x,y)=x2+xy+y2⁠, then the partial derivative of function f with respect to both variables x and y are, respectively:

∂f∂x=2x+y,∂f∂y=x+2y.

In general, the partial derivative of a function f(x1,…,xn) in the direction xi at the point (a1,…,an) is defined to be:

∂f∂xi(a1,…,an)=limh→0f(a1,…,ai+h,…,an)−f(a1,…,ai,…,an)h.

This is fundamental for the study of the functions of several real variables. Let f(x1,…,xn) be such a real-valued function. If all partial derivatives f with respect to xj are defined at the point ⁠(a1,…,an)⁠, these partial derivatives define the vector

∇f(a1,…,an)=(∂f∂x1(a1,…,an),…,∂f∂xn(a1,…,an)),

which is called the gradient of f at ⁠a⁠. If f is differentiable at every point in some domain, then the gradient is a vector-valued function ∇f that maps the point (a1,…,an) to the vector ⁠∇f(a1,…,an)⁠. Consequently, the gradient determines a vector field.

### Directional derivatives

If f is a real-valued function on ⁠ℝn⁠, then the partial derivatives of f measure its variation in the direction of the coordinate axes. For example, if f is a function of x and ⁠y⁠, then its partial derivatives measure the variation in f in the x and y direction. However, they do not directly measure the variation of f in any other direction, such as along the diagonal line ⁠y=x⁠. These are measured using directional derivatives. Given a vector ⁠𝐯=(v1,…,vn)⁠, then the directional derivative of f in the direction of 𝐯 at the point 𝐱 is:

D𝐯f(𝐱)=limh→0f(𝐱+h𝐯)−f(𝐱)h.

If all the partial derivatives of f exist and are continuous at ⁠𝐱⁠, then they determine the directional derivative of f in the direction 𝐯 by the formula:

D𝐯f(𝐱)=∑j=1nvj∂f∂xj.

### Total derivative and Jacobian matrix

When f is a function from an open subset of ℝn to ⁠ℝm⁠, then the directional derivative of f in a chosen direction is the best linear approximation to f at that point and in that direction. However, when ⁠n>1⁠, no single directional derivative can give a complete picture of the behavior of ⁠f⁠. The total derivative gives a complete picture by considering all directions at once. That is, for any vector 𝐯 starting at ⁠𝐚⁠, the linear approximation formula holds:

f(𝐚+𝐯)≈f(𝐚)+f′(𝐚)𝐯.

Similarly with the single-variable derivative, f′(𝐚) is chosen so that the error in this approximation is as small as possible. The total derivative of f at 𝐚 is the unique linear transformation f′(𝐚):ℝn→ℝm such that

lim𝐡→0‖f(𝐚+𝐡)−(f(𝐚)+f′(𝐚)𝐡)‖‖𝐡‖=0.

Here 𝐡 is a vector in ⁠ℝn⁠, so the norm in the denominator is the standard length on ⁠ℝn⁠. However, f′(𝐚)𝐡 is a vector in ⁠ℝm⁠, and the norm in the numerator is the standard length on ⁠ℝm⁠. If v is a vector starting at ⁠a⁠, then f′(𝐚)𝐯 is called the pushforward of 𝐯 by ⁠f⁠.

If the total derivative exists at ⁠𝐚⁠, then all the partial derivatives and directional derivatives of f exist at ⁠𝐚⁠, and for all ⁠𝐯⁠, f′(𝐚)𝐯 is the directional derivative of f in the direction ⁠𝐯⁠.  If f is written using coordinate functions, so that ⁠f=(f1,f2,…,fm)⁠, then the total derivative can be expressed using the partial derivatives as a matrix. This matrix is called the Jacobian matrix of f at 𝐚:

f′(𝐚)=Jac𝐚=(∂fi∂xj)ij.

## Generalizations

The concept of a derivative can be extended to many other settings. The common thread is that the derivative of a function at a point serves as a linear approximation of the function at that point.

An important generalization of the derivative concerns complex functions of complex variables, such as functions from (a domain in) the complex numbers ℂ to ⁠ℂ⁠. The notion of the derivative of such a function is obtained by replacing real variables with complex variables in the definition. If ℂ is identified with ℝ2 by writing a complex number z as ⁠x+iy⁠ then a differentiable function from ℂ to ℂ is certainly differentiable as a function from ℝ2 to ℝ2 (in the sense that its partial derivatives all exist), but the converse is not true in general: the complex derivative only exists if the real derivative is complex linear and this imposes relations between the partial derivatives called the Cauchy–Riemann equations – see holomorphic functions.

Another generalization concerns functions between differentiable or smooth manifolds. Intuitively speaking such a manifold M is a space that can be approximated near each point x by a vector space called its tangent space: the prototypical example is a smooth surface in ⁠ℝ3⁠. The derivative (or differential) of a (differentiable) map f:M→N between manifolds, at a point x in ⁠M⁠, is then a linear map from the tangent space of M at x to the tangent space of N at ⁠f(x)⁠. The derivative function becomes a map between the tangent bundles of M and ⁠N⁠. This definition is used in differential geometry.

Differentiation can also be defined for maps between vector space, such as Banach space, in which those generalizations are the Gateaux derivative and the Fréchet derivative.

One deficiency of the classical derivative is that very many functions are not differentiable. Nevertheless, there is a way of extending the notion of the derivative so that all continuous functions and many other functions can be differentiated using a concept known as the weak derivative. The idea is to embed the continuous functions in a larger space called the space of distributions and only require that a function is differentiable "on average".

Properties of the derivative have inspired the introduction and study of many similar objects in algebra and topology; an example is differential algebra. Here, it consists of the derivation of some topics in abstract algebra, such as rings, ideals, field, and so on.

The discrete equivalent of differentiation is finite differences. The study of differential calculus is unified with the calculus of finite differences in time scale calculus.

The arithmetic derivative involves the function that is defined for the integers by the prime factorization. This is an analogy with the product rule.

Derivations generalize derivatives to algebraic settings, such as rings.
