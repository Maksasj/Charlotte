# Exponential function

In mathematics, the exponential function is the unique real function which maps zero to one and has a derivative everywhere equal to its value. It is denoted ⁠ex⁠ or ⁠exp⁡x⁠; the latter is preferred when the argument ⁠x⁠ is a complicated expression. It is called exponential because its argument can be seen as an exponent to which a constant number e ≈ 2.718, the base, is raised. There are several other definitions of the exponential function, which are all equivalent although being of very different nature.

The exponential function converts sums to products: ⁠exp⁡(x+y)=exp⁡x⋅exp⁡y⁠. Its inverse function, the natural logarithm, ⁠ln⁠ or ⁠log⁠, converts products to sums: ⁠ln⁡(x⋅y)=ln⁡x+ln⁡y⁠.

The exponential function is occasionally called the natural exponential function, matching the name natural logarithm, for distinguishing it from some other functions that are also commonly called exponential functions. These functions include the functions of the form ⁠f(x)=bx⁠, which is exponentiation with a fixed base ⁠b⁠. More generally, and especially in applications, functions of the general form ⁠f(x)=abx⁠ are also called exponential functions. They grow or decay exponentially in that the rate that ⁠f(x)⁠ changes when ⁠x⁠ is increased is proportional to the current value of ⁠f(x)⁠.

The exponential function can be generalized to accept complex numbers as arguments. This reveals relations between multiplication of complex numbers, rotations in the complex plane, and trigonometry. Euler's formula ⁠eiθ=cos⁡θ+isin⁡θ⁠ expresses and summarizes these relations.

The exponential function can be even further generalized to accept other types of arguments, such as matrices and elements of Lie algebras.

## Graph

The graph of y=ex is upward-sloping, and increases faster than every power of ⁠x⁠. The graph always lies above the x-axis, but becomes arbitrarily close to it for large negative x; thus, the x-axis is a horizontal asymptote. The equation ddxex=ex means that the slope of the tangent to the graph at each point is equal to its height (its y-coordinate) at that point.

## Definitions and fundamental properties

There are several equivalent definitions of the exponential function, although of very different nature.

### Differential equation

The exponential function is the unique differentiable function that equals its derivative, and takes the value 1 for the value 0 of its variable.

This definition requires a uniqueness proof and an existence proof, but it allows an easy derivation of the main properties of the exponential function.

### Inverse of natural logarithm

The exponential function is the inverse function of the natural logarithm. That is,

ln⁡(exp⁡x)=xexp⁡(ln⁡y)=y

for every real number x and every positive real number y.

### Power series

The exponential function is the sum of the power series

exp⁡(x)=1+x+x22!+x33!+⋯=∑n=0∞xnn!,

where n! is the factorial of n (the product of the n first positive integers). This series is absolutely convergent for every x, by the ratio test. This shows that the exponential function is defined for every ⁠x⁠, and is everywhere the sum of its Maclaurin series.

### Functional equation

The exponential satisfies the functional equation

exp⁡(x+y)=exp⁡(x)⋅exp⁡(y)

and maps the additive identity 0 to the multiplicative identity 1.

The same equation is satisfied by other continuous functions f(x)=bx that exponentiate their argument with an arbitrary base b. Among these functions, the exponential function is characterized by the property that its derivative at 0 is 1.

### Limit of integer powers

The exponential function is the limit, as the integer n goes to infinity,

exp⁡(x)=limn→+∞(1+xn)n.

### Properties

Reciprocal: The functional equation implies ⁠exe−x=1⁠. Therefore ⁠ex≠0⁠ for every ⁠x⁠ and

1ex=e−x.

Positiveness: ⁠ex>0⁠ for every real number ⁠x⁠. This results from the intermediate value theorem, since ⁠e0=1⁠ and, if one would have ⁠ex<0⁠ for some ⁠x⁠, there would be an ⁠y⁠ such that ⁠ey=0⁠ between ⁠0⁠ and ⁠x⁠. Since the exponential function equals its derivative, this implies that the exponential function is monotonically increasing.

Extension of exponentiation to positive real bases: Let b be a positive real number. The exponential function and the natural logarithm being the inverse each of the other, one has b=exp⁡(ln⁡b). If n is an integer, the functional equation of the logarithm implies

bn=exp⁡(ln⁡bn)=exp⁡(nln⁡b).

Since the right-most expression is defined if n is any real number, this allows defining ⁠bx⁠ for every positive real number b and every real number x:

bx=exp⁡(xln⁡b).

In particular, if b is the Euler's number e=exp⁡(1), one has ln⁡e=1 (inverse function) and thus ex=exp⁡(x). This shows the equivalence of the two notations for the exponential function.

## General exponential functions

A function is commonly called an exponential function—with an indefinite article—if it has the form ⁠x↦bx⁠, that is, if it is obtained from exponentiation by fixing the base and letting the exponent vary.

More generally and especially in applied contexts, the term exponential function is commonly used for functions of the form ⁠f(x)=abx⁠. This may be motivated by the fact that, if the values of the function represent quantities, a change of measurement unit changes the value of ⁠a⁠, and so, it is nonsensical to impose ⁠a=1⁠.

These most general exponential functions are the differentiable functions that satisfy the following equivalent characterizations.

⁠f(x)=abx⁠ for every ⁠x⁠ and some constants ⁠a⁠ and ⁠b>0⁠.

⁠f(x)=aekx⁠ for every ⁠x⁠ and some constants ⁠a⁠ and ⁠k⁠.

The value of f′(x)/f(x)  is independent of x.

For every d, the value of f(x+d)/f(x) is independent of x; that is, f(x+d)f(x)=f(y+d)f(y) for every x, y.

The base of an exponential function is the base of the exponentiation that appears in it when written as ⁠x→abx⁠, namely ⁠b⁠. The base is ⁠ek⁠ in the second characterization, exp⁡f′(x)f(x) in the third one, and (f(x+d)f(x))1/d in the last one.

### In applications

The last characterization is important in empirical sciences, as allowing a direct experimental test whether a function is an exponential function.

Exponential growth or exponential decay—where the variable change is proportional to the variable value—are thus modeled with exponential functions. Examples are unlimited population growth leading to Malthusian catastrophe, continuously compounded interest, and radioactive decay.

If the modeling function has the form ⁠x↦aekx,⁠ or, equivalently, is a solution of the differential equation ⁠y′=ky⁠, the constant ⁠k⁠ is called, depending on the context, the decay constant, disintegration constant, rate constant, or transformation constant.

### Equivalence proof

For proving the equivalence of the above properties, one can proceed as follows.

The two first characterizations are equivalent, since, if ⁠b=ek⁠ and ⁠k=ln⁡b⁠, one has

ekx=(ek)x=bx.

The basic properties of the exponential function (derivative and functional equation) implies immediately the third and the last condition.

Suppose that the third condition is verified, and let ⁠k⁠ be the constant value of f′(x)/f(x). Since ∂ekx∂x=kekx, the quotient rule for derivation

implies that

∂∂xf(x)ekx=0, and thus that there is a constant ⁠a⁠ such that f(x)=aekx.

If the last condition is verified, let φ(d)=f(x+d)/f(x), which is independent of ⁠x⁠. Using ⁠φ(0)=1⁠, one gets

f(x+d)−f(x)d=f(x)φ(d)−φ(0)d.

Taking the limit when ⁠d⁠ tends to zero, one gets that the third condition is verified with ⁠k=φ′(0)⁠. It follows therefore that ⁠f(x)=aekx⁠ for some ⁠a,⁠ and ⁠φ(d)=ekd.⁠ As a byproduct, one gets that

(f(x+d)f(x))1/d=ek

is independent of both ⁠x⁠ and ⁠d⁠.

## Compound interest

The earliest occurrence of the exponential function was in Jacob Bernoulli's study of compound interests in 1683.

This is this study that led Bernoulli to consider the number

limn→∞(1+1n)n

now known as Euler's number and denoted ⁠e⁠.

The exponential function is involved as follows in the computation of continuously compounded interests.

If a principal amount of 1 earns interest at an annual rate of x compounded monthly, then the interest earned each month is ⁠x/12⁠ times the current value, so each month the total value is multiplied by (1 + ⁠x/12⁠), and the value at the end of the year is (1 + ⁠x/12⁠)12.  If instead interest is compounded daily, this becomes (1 + ⁠x/365⁠)365.  Letting the number of time intervals per year grow without bound leads to the limit definition of the exponential function,

exp⁡x=limn→∞(1+xn)n

first given by Leonhard Euler.

## Differential equations

Exponential functions occur very often in solutions of differential equations.

The exponential functions can be defined as solutions of differential equations. Indeed, the exponential function is a solution of the simplest possible differential equation, namely ⁠y′=y⁠. Every other exponential function, of the form ⁠y=abx⁠, is a solution of the differential equation ⁠y′=ky⁠, and every solution of this differential equation has this form.

The solutions of an equation of the form

y′+ky=f(x)

involve exponential functions in a more sophisticated way, since they have the form

y=ce−kx+e−kx∫f(x)ekxdx,

where ⁠c⁠ is an arbitrary constant and the integral denotes any antiderivative of its argument.

More generally, the solutions of every linear differential equation with constant coefficients can be expressed in terms of exponential functions and, when they are not homogeneous, antiderivatives. This holds true also for systems of linear differential equations with constant coefficients.

## Complex exponential

The exponential function can be naturally extended to a complex function, which is a function with the complex numbers as domain and codomain, such that its restriction to the reals is the above-defined exponential function, called real exponential function in what follows. This function is also called the exponential function, and also denoted ⁠ez⁠ or ⁠exp⁡(z)⁠. For distinguishing the complex case from the real one, the extended function is also called complex exponential function or simply complex exponential.

Most of the definitions of the exponential function can be used verbatim for definiting the complex exponential function, and the proof of their equivalence is the same as in the real case.

The complex exponential function can be defined in several equivalent ways that are the same as in the real case.

The complex exponential is the unique complex function that equals its complex derivative and takes the value ⁠1⁠ for the argument ⁠0⁠:

dezdz=ezande0=1.

The complex exponential function is the sum of the series

ez=∑k=0∞zkk!.

This series is absolutely convergent for every complex number ⁠z⁠. So, the complex exponential is an entire function.

The complex exponential function is the limit

ez=limn→∞(1+zn)n

As with the real exponential function (see § Functional equation above), the complex exponential satisfies the functional equation

exp⁡(z+w)=exp⁡(z)⋅exp⁡(w).

Among complex functions, it is the unique solution which is holomorphic at the point ⁠z=0⁠ and takes the derivative ⁠1⁠ there.

The complex logarithm is a right-inverse function of the complex exponential:

elog⁡z=z.

However, since the complex logarithm is a multivalued function, one has

log⁡ez={z+2ikπ∣k∈ℤ},

and it is difficult to define the complex exponential from the complex logarithm. On the opposite, this is the complex logarithm that is often defined from the complex exponential.

The complex exponential has the following properties:

1ez=e−z

and

ez≠0for every z∈ℂ.

It is a periodic function of period ⁠2iπ⁠; that is

ez+2ikπ=ezfor every k∈ℤ.

This results from Euler's identity ⁠eiπ=−1⁠ and the functional identity.

The complex conjugate of the complex exponential is

ez‾=ez‾.

Its modulus is

|ez|=eℜ(z),

where ⁠ℜ(z)⁠ denotes the real part of ⁠z⁠.

### Relationship with trigonometry

Complex exponential and trigonometric functions are strongly related by Euler's formula:

eit=cos⁡(t)+isin⁡(t).

This formula provides the decomposition of complex exponentials into real and imaginary parts:

ex+iy=exeiy=excos⁡y+iexsin⁡y.

The trigonometric functions can be expressed in terms of complex exponentials:

cos⁡x=eix+e−ix2sin⁡x=eix−e−ix2itan⁡x=i1−e2ix1+e2ix

In these formulas, ⁠x,y,t⁠ are commonly interpreted as real variables, but the formulas remain valid if the variables are interpreted as complex variables. These formulas may be used to define trigonometric functions of a complex variable.

### Plots

Considering the complex exponential function as a function involving four real variables:

v+iw=exp⁡(x+iy)

the graph of the exponential function is a two-dimensional surface curving through four dimensions.

Starting with a color-coded portion of the xy domain, the following are depictions of the graph as variously projected into two or three dimensions.

The second image shows how the domain complex plane is mapped into the range complex plane:

zero is mapped to 1

the real x axis is mapped to the positive real v axis

the imaginary y axis is wrapped around the unit circle at a constant angular rate

values with negative real parts are mapped inside the unit circle

values with positive real parts are mapped outside of the unit circle

values with a constant real part are mapped to circles centered at zero

values with a constant imaginary part are mapped to rays extending from zero

The third and fourth images show how the graph in the second image extends into one of the other two dimensions not shown in the second image.

The third image shows the graph extended along the real x axis.  It shows the graph is a surface of revolution about the x axis of the graph of the real exponential function, producing a horn or funnel shape.

The fourth image shows the graph extended along the imaginary y axis.  It shows that the graph's surface for positive and negative y values doesn't really meet along the negative real v axis, but instead forms a spiral surface about the y axis.  Because its y values have been extended to ±2π, this image also better depicts the 2π periodicity in the imaginary y value.

## Transcendency

The function ez is a transcendental function, which means that it is not a root of a polynomial over the field of the rational fractions ℂ(z); in fact, this is true for any exponential function with a positive real base not equal to 1.

This follows from the stronger statement that if a1, ..., an are distinct complex numbers, then ea1z, ..., eanz are linearly independent over ℂ(z).

A much more difficult result is that the base e of the natural exponential function is a transcendental number, see the Lindemann–Weierstrass theorem.

## Computation

The Taylor series definition above is generally efficient for computing (an approximation of) ex. However, when computing near the argument x=0, the result will be close to 1, and computing the value of the difference ex−1 with floating-point arithmetic may lead to the loss of (possibly all) significant figures, producing a large relative error, possibly even a meaningless result.

Following a proposal by William Kahan, it may thus be useful to have a dedicated routine, often called expm1, which computes  ex − 1 directly, bypassing computation of ex. For example,

one may use the Taylor series:

ex−1=x+x22+x36+⋯+xnn!+⋯.

This was first implemented in 1979 in the Hewlett-Packard HP-41C calculator, and provided by several calculators, operating systems (for example Berkeley UNIX 4.3BSD), computer algebra systems, and programming languages (for example C99).

In addition to base e, the IEEE 754-2008 standard defines similar exponential functions near 0 for base 2 and 10: 2x−1 and 10x−1.

A similar approach has been used for the logarithm; see log1p.

An identity in terms of the hyperbolic tangent,

expm1⁡(x)=ex−1=2tanh⁡(x/2)1−tanh⁡(x/2),

gives a high-precision value for small values of x on systems that do not implement expm1(x).

### Continued fractions

The exponential function can also be computed with continued fractions.

A continued fraction for ex can be obtained via an identity of Euler:

ex=1+x1−xx+2−2xx+3−3xx+4−⋱

The following generalized continued fraction for ez, also due to Euler,

converges more quickly:

ez=1+2z2−z+z26+z210+z214+⋱

or, by applying the substitution z = ⁠x/y⁠:

exy=1+2x2y−x+x26y+x210y+x214y+⋱

with a special case for z = 2:

e2=1+40+226+2210+2214+⋱=7+25+17+19+111+⋱

This formula also converges, though more slowly, for z > 2. For example:

e3=1+6−1+326+3210+3214+⋱=13+547+914+918+922+⋱

### Matrices and Banach algebras

The power series definition of the exponential function makes sense for square matrices (for which the function is called the matrix exponential) and more generally in any unital Banach algebra B.  In this setting, e0 = 1, and ex is invertible with inverse e−x for any x in B.  If xy = yx, then ex + y = exey, but this identity can fail for noncommuting x and y.

Some alternative definitions lead to the same function.  For instance, ex can be defined as

limn→∞(1+xn)n.

Or ex can be defined as fx(1), where fx : R → B is the solution to the differential equation ⁠dfx/dt⁠(t) = x fx(t), with initial condition fx(0) = 1; it follows that fx(t) = etx for every t in R.

### Lie algebras

Given a Lie group G and its associated Lie algebra 𝔤, the exponential map is a map 𝔤→G satisfying similar properties. In fact, since R is the Lie algebra of the Lie group of all positive real numbers under multiplication, the ordinary exponential function for real arguments is a special case of the Lie algebra situation. Similarly, since the Lie group GL(n,R) of invertible n × n matrices has as Lie algebra M(n,R), the space of all n × n matrices, the exponential function for square matrices is a special case of the Lie algebra exponential map.

The identity exp⁡(x+y)=exp⁡(x)exp⁡(y) can fail for Lie algebra elements x and y that do not commute; the Baker–Campbell–Hausdorff formula supplies the necessary correction terms.
