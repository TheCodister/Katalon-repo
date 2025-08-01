<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_ModalDisplays a dialog with custom cont_bbcc0b</name>
   <tag></tag>
   <elementGuidId>a27b59f5-bf90-49ab-8f85-0e8b8841b104</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value></value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.mdx</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Modal Displays a dialog with custom content that requires attention or provides &quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>cb8198e4-ee68-4781-bc7f-02c6caf063a5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>mdx</value>
      <webElementGuid>2027a13c-fafe-406c-ac59-dda8111f3f97</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Modal
Displays a dialog with custom content that requires attention or provides additional information.
Storybook@heroui/modalReact AriaSourceStyles source


Installation
CLIpnpmnpmyarnbunThe above command is for individual installation only. You may skip this step if @heroui/react is already installed globally.
Import
HeroUI exports 5 modal-related components:

Modal: The main component to display a modal.
ModalContent: The wrapper of the other modal components.
ModalHeader: The header of the modal.
ModalBody: The body of the modal.
ModalFooter: The footer of the modal.

IndividualGlobalimport {  Modal,  ModalContent,  ModalHeader,  ModalBody,  ModalFooter} from &quot;@heroui/modal&quot;;
Usage
When the modal opens:

Focus is bounded within the modal and set to the first tabbable element.
Content behind the modal dialog is inert, meaning that users cannot interact with it.

PreviewCodeOpen ModalOpen in Chat 
Sizes
PreviewCodeOpen xsOpen smOpen mdOpen lgOpen xlOpen 2xlOpen 3xlOpen 4xlOpen 5xlOpen fullOpen in Chat 
Non-dismissible
By default, the modal can be closed by clicking on the overlay or pressing the Esc key.
You can disable this behavior by setting the following properties:

Set the isDismissable property to false to prevent the modal from closing when clicking on the overlay.
Set the isKeyboardDismissDisabled property to true to prevent the modal from closing when pressing the Esc key.

PreviewCodeOpen ModalOpen in Chat 
Modal placement
By default the modal is centered on screens larger than sm and is at the bottom of the screen on mobile. This placement is called auto, but
you can change it by using the placement prop.
PreviewCodeOpen in Chat 

Note: The top-center and bottom-center positions mean that the modal is positioned at the top / bottom of the screen
on mobile, and at the center of the screen on desktop.

Overflow scroll
You can use the scrollBehavior prop to set the scroll behavior of the modal.

inside: The modal content will be scrollable.
outside: The modal content will be scrollable and the modal will be fixed.

PreviewCodeOpen ModalSelect scroll behaviorinsideoutsideOpen in Chat 
With Form
The Modal handles the focus within the modal content. It means that you can use the modal with
form elements without any problem. The focus returns to the trigger when the modal closes.
PreviewCodeOpen ModalOpen in Chat 

Note: You can add the autoFocus prop to the first Input component to focus it when the modal opens.

Backdrop
The Modal component has a backdrop prop to show a backdrop behind the modal. The backdrop can be
either transparent, opaque or blur. The default value is opaque.
PreviewCodeopaqueblurtransparentOpen in Chat 
Custom Backdrop
You can customize the backdrop by using the backdrop slot.
PreviewCodeOpen ModalOpen in Chat 
Custom Motion
Modal offers a motionProps property to customize the enter / exit animation.
PreviewCodeOpen ModalOpen in Chat 

Learn more about Framer motion variants here.

Draggable
Try to drag the modal by clicking on the modal header and dragging.
PreviewCodeOpen ModalOpen in Chat 
Draggable Overflow
Setting overflow to true allows users to drag the modal to a position where it overflows the viewport.
PreviewCodeOpen ModalOpen in Chat 
Slots

wrapper: The wrapper slot of the modal. It wraps the base and the backdrop slots.
base: The main slot of the modal content.
backdrop: The backdrop slot, it is displayed behind the modal.
header: The header of the modal, it is displayed at the top of the modal.
body: The body of the modal, it is displayed in the middle of the modal.
footer: The footer of the modal, it is displayed at the bottom of the modal.
closeButton: The close button of the modal.

Custom Styles
You can customize the Modal component by passing custom Tailwind CSS classes to the component slots.
PreviewCodeOpen ModalOpen in Chat 

Data Attributes
Modal has the following attributes on the base element:

data-open:
When the modal is open. Based on modal state.
data-dismissable:
When the modal is dismissable. Based on isDismissable prop.


Accessibility

Content outside the modal is hidden from assistive technologies while it is open.
The modal optionally closes when interacting outside, or pressing the Esc key.
Focus is moved into the modal on mount, and restored to the trigger element on unmount.
While open, focus is contained within the modal, preventing the user from tabbing outside.
Scrolling the page behind the modal is prevented while it is open, including in mobile browsers.


API
Modal Props
PropTypeDefaultchildren*ReactNodesizexs | sm | md | lg | xl | 2xl | 3xl | 4xl | 5xl | full&quot;md&quot;radiusnone | sm | md | lg&quot;lg&quot;shadownone | sm | md | lg&quot;lg&quot;backdroptransparent | opaque | blur&quot;opaque&quot;scrollBehaviornormal | inside | outside&quot;normal&quot;placementauto | top | center | bottom&quot;auto&quot;isOpenbooleandefaultOpenbooleanisDismissablebooleantrueisKeyboardDismissDisabledbooleanfalseshouldBlockScrollbooleantruehideCloseButtonbooleanfalsecloseButtonReactNodemotionPropsMotionPropsportalContainerHTMLElement&quot;document.body&quot;disableAnimationbooleanfalseclassNamesPartial&lt;Record&lt;'wrapper' | 'base' | 'backdrop' | 'header' | 'body' | 'footer' | 'closeButton', string>>
Modal Events
PropTypeDefaultonOpenChange(isOpen: boolean) => voidonClose() => void

Modal types
Motion Props
</value>
      <webElementGuid>65b4b852-0146-492d-abde-1cddf09cec75</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;app-container&quot;)/main[@class=&quot;relative container mx-auto max-w-8xl z-10 px-6 min-h-[calc(100vh_-_64px_-_108px)] mb-12 grow&quot;]/div[@class=&quot;grid grid-cols-12&quot;]/div[@class=&quot;col-span-12 lg:col-span-10 xl:col-span-8 lg:px-16 mt-10&quot;]/div[@class=&quot;w-full prose prose-neutral&quot;]/div[@class=&quot;mdx&quot;]</value>
      <webElementGuid>44e93221-243b-417e-8c48-4248057d92bc</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='app-container']/main/div/div[2]/div/div</value>
      <webElementGuid>ad98c582-b9c2-4451-ac6a-cab2ec4f7425</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='HeroUIProvider'])[1]/following::div[3]</value>
      <webElementGuid>2b550199-8f05-4b8e-b52f-cb11cd658e7c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='HeroUI CLI'])[1]/following::div[4]</value>
      <webElementGuid>31152146-19e2-4c69-924e-109f8ad3acce</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>7d36d54c-74e9-4568-9d29-5c5bceb10b86</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div</value>
      <webElementGuid>820e076e-a3f6-43e1-b039-27ff85dee882</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;Modal
Displays a dialog with custom content that requires attention or provides additional information.
Storybook@heroui/modalReact AriaSourceStyles source


Installation
CLIpnpmnpmyarnbunThe above command is for individual installation only. You may skip this step if @heroui/react is already installed globally.
Import
HeroUI exports 5 modal-related components:

Modal: The main component to display a modal.
ModalContent: The wrapper of the other modal components.
ModalHeader: The header of the modal.
ModalBody: The body of the modal.
ModalFooter: The footer of the modal.

IndividualGlobalimport {  Modal,  ModalContent,  ModalHeader,  ModalBody,  ModalFooter} from &quot;@heroui/modal&quot;;
Usage
When the modal opens:

Focus is bounded within the modal and set to the first tabbable element.
Content behind the modal dialog is inert, meaning that users cannot interact with it.

PreviewCodeOpen ModalOpen in Chat 
Sizes
PreviewCodeOpen xsOpen smOpen mdOpen lgOpen xlOpen 2xlOpen 3xlOpen 4xlOpen 5xlOpen fullOpen in Chat 
Non-dismissible
By default, the modal can be closed by clicking on the overlay or pressing the Esc key.
You can disable this behavior by setting the following properties:

Set the isDismissable property to false to prevent the modal from closing when clicking on the overlay.
Set the isKeyboardDismissDisabled property to true to prevent the modal from closing when pressing the Esc key.

PreviewCodeOpen ModalOpen in Chat 
Modal placement
By default the modal is centered on screens larger than sm and is at the bottom of the screen on mobile. This placement is called auto, but
you can change it by using the placement prop.
PreviewCodeOpen in Chat 

Note: The top-center and bottom-center positions mean that the modal is positioned at the top / bottom of the screen
on mobile, and at the center of the screen on desktop.

Overflow scroll
You can use the scrollBehavior prop to set the scroll behavior of the modal.

inside: The modal content will be scrollable.
outside: The modal content will be scrollable and the modal will be fixed.

PreviewCodeOpen ModalSelect scroll behaviorinsideoutsideOpen in Chat 
With Form
The Modal handles the focus within the modal content. It means that you can use the modal with
form elements without any problem. The focus returns to the trigger when the modal closes.
PreviewCodeOpen ModalOpen in Chat 

Note: You can add the autoFocus prop to the first Input component to focus it when the modal opens.

Backdrop
The Modal component has a backdrop prop to show a backdrop behind the modal. The backdrop can be
either transparent, opaque or blur. The default value is opaque.
PreviewCodeopaqueblurtransparentOpen in Chat 
Custom Backdrop
You can customize the backdrop by using the backdrop slot.
PreviewCodeOpen ModalOpen in Chat 
Custom Motion
Modal offers a motionProps property to customize the enter / exit animation.
PreviewCodeOpen ModalOpen in Chat 

Learn more about Framer motion variants here.

Draggable
Try to drag the modal by clicking on the modal header and dragging.
PreviewCodeOpen ModalOpen in Chat 
Draggable Overflow
Setting overflow to true allows users to drag the modal to a position where it overflows the viewport.
PreviewCodeOpen ModalOpen in Chat 
Slots

wrapper: The wrapper slot of the modal. It wraps the base and the backdrop slots.
base: The main slot of the modal content.
backdrop: The backdrop slot, it is displayed behind the modal.
header: The header of the modal, it is displayed at the top of the modal.
body: The body of the modal, it is displayed in the middle of the modal.
footer: The footer of the modal, it is displayed at the bottom of the modal.
closeButton: The close button of the modal.

Custom Styles
You can customize the Modal component by passing custom Tailwind CSS classes to the component slots.
PreviewCodeOpen ModalOpen in Chat 

Data Attributes
Modal has the following attributes on the base element:

data-open:
When the modal is open. Based on modal state.
data-dismissable:
When the modal is dismissable. Based on isDismissable prop.


Accessibility

Content outside the modal is hidden from assistive technologies while it is open.
The modal optionally closes when interacting outside, or pressing the Esc key.
Focus is moved into the modal on mount, and restored to the trigger element on unmount.
While open, focus is contained within the modal, preventing the user from tabbing outside.
Scrolling the page behind the modal is prevented while it is open, including in mobile browsers.


API
Modal Props
PropTypeDefaultchildren*ReactNodesizexs | sm | md | lg | xl | 2xl | 3xl | 4xl | 5xl | full&quot;md&quot;radiusnone | sm | md | lg&quot;lg&quot;shadownone | sm | md | lg&quot;lg&quot;backdroptransparent | opaque | blur&quot;opaque&quot;scrollBehaviornormal | inside | outside&quot;normal&quot;placementauto | top | center | bottom&quot;auto&quot;isOpenbooleandefaultOpenbooleanisDismissablebooleantrueisKeyboardDismissDisabledbooleanfalseshouldBlockScrollbooleantruehideCloseButtonbooleanfalsecloseButtonReactNodemotionPropsMotionPropsportalContainerHTMLElement&quot;document.body&quot;disableAnimationbooleanfalseclassNamesPartial&lt;Record&lt;&quot; , &quot;'&quot; , &quot;wrapper&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;base&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;backdrop&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;closeButton&quot; , &quot;'&quot; , &quot;, string>>
Modal Events
PropTypeDefaultonOpenChange(isOpen: boolean) => voidonClose() => void

Modal types
Motion Props
&quot;) or . = concat(&quot;Modal
Displays a dialog with custom content that requires attention or provides additional information.
Storybook@heroui/modalReact AriaSourceStyles source


Installation
CLIpnpmnpmyarnbunThe above command is for individual installation only. You may skip this step if @heroui/react is already installed globally.
Import
HeroUI exports 5 modal-related components:

Modal: The main component to display a modal.
ModalContent: The wrapper of the other modal components.
ModalHeader: The header of the modal.
ModalBody: The body of the modal.
ModalFooter: The footer of the modal.

IndividualGlobalimport {  Modal,  ModalContent,  ModalHeader,  ModalBody,  ModalFooter} from &quot;@heroui/modal&quot;;
Usage
When the modal opens:

Focus is bounded within the modal and set to the first tabbable element.
Content behind the modal dialog is inert, meaning that users cannot interact with it.

PreviewCodeOpen ModalOpen in Chat 
Sizes
PreviewCodeOpen xsOpen smOpen mdOpen lgOpen xlOpen 2xlOpen 3xlOpen 4xlOpen 5xlOpen fullOpen in Chat 
Non-dismissible
By default, the modal can be closed by clicking on the overlay or pressing the Esc key.
You can disable this behavior by setting the following properties:

Set the isDismissable property to false to prevent the modal from closing when clicking on the overlay.
Set the isKeyboardDismissDisabled property to true to prevent the modal from closing when pressing the Esc key.

PreviewCodeOpen ModalOpen in Chat 
Modal placement
By default the modal is centered on screens larger than sm and is at the bottom of the screen on mobile. This placement is called auto, but
you can change it by using the placement prop.
PreviewCodeOpen in Chat 

Note: The top-center and bottom-center positions mean that the modal is positioned at the top / bottom of the screen
on mobile, and at the center of the screen on desktop.

Overflow scroll
You can use the scrollBehavior prop to set the scroll behavior of the modal.

inside: The modal content will be scrollable.
outside: The modal content will be scrollable and the modal will be fixed.

PreviewCodeOpen ModalSelect scroll behaviorinsideoutsideOpen in Chat 
With Form
The Modal handles the focus within the modal content. It means that you can use the modal with
form elements without any problem. The focus returns to the trigger when the modal closes.
PreviewCodeOpen ModalOpen in Chat 

Note: You can add the autoFocus prop to the first Input component to focus it when the modal opens.

Backdrop
The Modal component has a backdrop prop to show a backdrop behind the modal. The backdrop can be
either transparent, opaque or blur. The default value is opaque.
PreviewCodeopaqueblurtransparentOpen in Chat 
Custom Backdrop
You can customize the backdrop by using the backdrop slot.
PreviewCodeOpen ModalOpen in Chat 
Custom Motion
Modal offers a motionProps property to customize the enter / exit animation.
PreviewCodeOpen ModalOpen in Chat 

Learn more about Framer motion variants here.

Draggable
Try to drag the modal by clicking on the modal header and dragging.
PreviewCodeOpen ModalOpen in Chat 
Draggable Overflow
Setting overflow to true allows users to drag the modal to a position where it overflows the viewport.
PreviewCodeOpen ModalOpen in Chat 
Slots

wrapper: The wrapper slot of the modal. It wraps the base and the backdrop slots.
base: The main slot of the modal content.
backdrop: The backdrop slot, it is displayed behind the modal.
header: The header of the modal, it is displayed at the top of the modal.
body: The body of the modal, it is displayed in the middle of the modal.
footer: The footer of the modal, it is displayed at the bottom of the modal.
closeButton: The close button of the modal.

Custom Styles
You can customize the Modal component by passing custom Tailwind CSS classes to the component slots.
PreviewCodeOpen ModalOpen in Chat 

Data Attributes
Modal has the following attributes on the base element:

data-open:
When the modal is open. Based on modal state.
data-dismissable:
When the modal is dismissable. Based on isDismissable prop.


Accessibility

Content outside the modal is hidden from assistive technologies while it is open.
The modal optionally closes when interacting outside, or pressing the Esc key.
Focus is moved into the modal on mount, and restored to the trigger element on unmount.
While open, focus is contained within the modal, preventing the user from tabbing outside.
Scrolling the page behind the modal is prevented while it is open, including in mobile browsers.


API
Modal Props
PropTypeDefaultchildren*ReactNodesizexs | sm | md | lg | xl | 2xl | 3xl | 4xl | 5xl | full&quot;md&quot;radiusnone | sm | md | lg&quot;lg&quot;shadownone | sm | md | lg&quot;lg&quot;backdroptransparent | opaque | blur&quot;opaque&quot;scrollBehaviornormal | inside | outside&quot;normal&quot;placementauto | top | center | bottom&quot;auto&quot;isOpenbooleandefaultOpenbooleanisDismissablebooleantrueisKeyboardDismissDisabledbooleanfalseshouldBlockScrollbooleantruehideCloseButtonbooleanfalsecloseButtonReactNodemotionPropsMotionPropsportalContainerHTMLElement&quot;document.body&quot;disableAnimationbooleanfalseclassNamesPartial&lt;Record&lt;&quot; , &quot;'&quot; , &quot;wrapper&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;base&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;backdrop&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;header&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;footer&quot; , &quot;'&quot; , &quot; | &quot; , &quot;'&quot; , &quot;closeButton&quot; , &quot;'&quot; , &quot;, string>>
Modal Events
PropTypeDefaultonOpenChange(isOpen: boolean) => voidonClose() => void

Modal types
Motion Props
&quot;))]</value>
      <webElementGuid>3f790e3a-e0a9-475b-b46e-745f523f010a</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
